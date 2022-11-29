# Assignment 2: Rusttracer (Performance)
This is assignment 2, about performance. This file records every benchmark with both using time and criterion.



## I. Checklist
### Baseline Requirements:
- [x] Apply at least 3 of the performance improvement techniques discussed in class
- [x] Apply at least 5 (distinct) performance optimizations
- [x] Write at least 1 benchmark using criterion to evaluate the performance of your changes.
- Write a report containing, for every optimization, write at least 100 words in which you:
	- [x] Describe what it is and how it works
	- [x] Describe why it makes the code faster
	- [x] Describe how you discovered this optimization
	- [x] Show some kind of benchmark (does not need to be with criterion) which proves it made the code faster compared to a previous version of your code
- [x] In what order you discovered each of the optimizations
- [x] What hardware you used to benchmark, and in what way your hardware affects the benchmarks you ran

### Extra Requirements:
- [x] Find more ways to increase the performance of the code (depending on the complexity, your description and the effect it has, roughly 1 point per optimization)
- [x] Separate benchmarks (using criterion) for each of your optimizations and the effect they have compared to the baseline (1 point)
- [x] After you hand your assignment in, we will benchmark your code against our original. If your code is faster than the reference you get 0.5 bonus. You can ask TAs to benchmark your code before you hand in.

## II. Record of Optimization

All benchmark of all optimizations use: 
```rust=
time cargo run --release
```

All benchmark in this file are benchmarked after every optimization is done and before getting the final codes which might be different from benchmark using criterion in the end.

### 1. Function set_at() optimization (I/O buffering, get rid of things from for-loop)
![](https://i.imgur.com/n0cH5xT.png)
Because the first version takes a lot of time, so 100*100 100 is used.

Graph size: 100 * 100 
Sample rate:100
**Benchmark before improvement: 10min 7s
Benchmark after improvement: 4.5s**

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after improvement: 3min 41s**, even with bigger graph, it is still faster
```rust=
    pub fn set_at_threaded(&mut self, x: usize, y: usize, color: Vector, backup_file: &mut File) {
        self.buffer[y][x] = color;

        let mut buf = BufWriter::new(backup_file);
        write!(buf, "{}, {}, {};", color.x, color.y, color.z);
        if x == self.buffer.len() - 1 {
            writeln!(buf);
        }
        buf.flush();
    }
```
![](https://i.imgur.com/BYj7KB0.png)
![](https://i.imgur.com/HBnxngg.png)

### 2. Function intersects_triangle() and boundingbox::contains() optimizations (get rid of function calls, inline)
#### Optimizations in intersects_triangle()
```rust=
    let aa = triangle.a();
    let edge1 = triangle.b() - aa;
    let edge2 = triangle.c() - aa;
```

#### Optimizations in contains()
```rust=
    let a = triangle.a();
    let b = triangle.b();
    let c = triangle.c();
    if a.x < self.min.x && b.x < self.min.x && c.x < self.min.x
    {
        return false;
    }
    if a.y < self.min.y && b.y < self.min.y && c.y < self.min.y
    {
        return false;
    }
```

#### Optimizations of Triangle.a(), b(), c()
```rust=
    #[inline]
    pub fn a(&self) -> Vector {
        self.mesh.vertices[self.a]
    }
    #[inline]
    pub fn b(&self) -> Vector {
        self.mesh.vertices[self.b]
    }
    #[inline]
    pub fn c(&self) -> Vector {
        self.mesh.vertices[self.c]
    }
```
Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 3min 38s
Improvement: 3s**

![](https://i.imgur.com/bf7gra9.png)
And now a has almost same calling time as b and c

### 3. Function datastructure::intersects_trangle() improved with Vector::dot() Vector::cross() optimizations (pass by reference)
In util/vector.rs, function like dot() and cross() are changed from passing by value to passing by reference
![](https://i.imgur.com/4SHqAW7.png)

intersects_trangle() takes less percentage in the whole program which is 5% less
![](https://i.imgur.com/0ovGyp9.png)

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 3min 36s
Improvement: 2s**


### 4. Function mstracer::raytrace() optimization 1 (move things out of loops)
Because according to the flamegraph, there are two recursive function calls take the most time, intersect_internal() and shade_internal(). So shade_internal() and functions which call this function are checked. 

The raytrace() is one of functions which calls shade_internal() indirectly. Before the function shade_internal() is optimized, some redundant things in for loop are moved out of loops.

Before optimization:
```rust=
    let mut out = Vector::repeated(0f64);
    for _ in 0..self.samples_per_pixel {
        let ray = camera.generate_ray(x as f64, y as f64);
        out += shader.shade(&ray, datastructure.clone()) / 
                                 self.samples_per_pixel as f64;
        print!("\r{x}, {y} ");
        stdout().flush().unwrap();
    }
    out
```

After optimization:
```rust=
    let mut out = Vector::repeated(0f64);
    let ray = camera.generate_ray(x as f64, y as f64);
    for _ in 0..self.samples_per_pixel {
        out += shader.shade(&ray, datastructure.clone());
    }
    print!("\r{x}, {y} ");
    stdout().flush().unwrap();
    out/ self.samples_per_pixel as f64
```
Less devisions and prints and function calls are done in the loop.

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 2min 36s
Improvement: 60s**

### 5. Function mstracer::raytraycer() optimization 2 with function shade_internal() and deleting Mutex (get rid of if-else judgement in recursive calls and for loop, get rid of Mutex locks, get rid of branches)

The following if-statement is in the function shade_internal(). shade_internel() is called by shade() and itself with recursive calls, and also shade() is called in the a for-loop of function raytrace() which is mentioned in last chapter. So move this if-statement out of the function and for-loop can get rid of branch judgement with branch pridictions.
```rust=
    let intersection =
        if let Some(intersection) = 
    datastructure.lock().unwrap().intersects(*ray.clone()) {
            intersection
        } else {
            return Vector::repeated(0f64);
        };
```

And also, the value datastructure is always passed as Arc(Mutex(Box(dyn DataStructure))), sometimes lock() and unwrap() is called which waste time. And also, value of datastructure is never changed in the whole program. So Mutex and Box should be removed.
    
Like this:
```rust=
pub fn shade_internal(
    &self,
    ray: &Box<Ray>,
    depth: usize,
    datastructure: Arc<dyn DataStructure>,
    intersection: &Option<Intersection>,
)
```
    
And after improvement, the raytrace() should be like this:
```rust=
    let mut out = Vector::repeated(0f64);
    let ray = camera.generate_ray(x as f64, y as f64);
    let intersection = datastructure.intersects(&ray);
    for _ in 0..self.samples_per_pixel {
        out += shader.shade(&ray, datastructure.clone(), &intersection);
    }

    print!("\r{x}, {y} ");
    stdout().flush().unwrap();
    out / self.samples_per_pixel as f64
```

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 2min 22s
Improvement: 14s**
    
### 6. Function generator::generate() optimization(get rid of lock contention)
In function generate() in threads.rs, the variable callback(x, y) is always used after a Mutex lock which causes a lot lock contentions. So the code is changed like this:

Before optimization:
```rust=
for x in 0..camera.width {
    let mut guard = local_output.lock().unwrap();
    guard.set_at_threaded(x, y, callback(x, y), &mut backup_file);
}
```

After optimization:
```rust=
for x in 0..camera.width {
    let color = callback(x, y);
    let mut guard = local_output.lock().unwrap();
    guard.set_at_threaded(x, y, color, &mut backup_file);
}
```

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 37.6s
Improvement: 1min 44.4s**


### 7. Function Texture::new() optimization (I/O buffering)

In function Texture::new(), it directly reads from the file. An i/o buffer can be used.

Before optimization:
```rust=
    let mut f = File::open(filename)?;
    let mut buf = [0; 20];
    let mut image_buffer = Vec::with_capacity(0);
    while f.read_exact(&mut buf).is_ok() {
        for i in buf {
            image_buffer.push(i);
        }
    }
```

After optimization:
```rust=
        let mut f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut buf = [0; 20];
        let mut image_buffer = Vec::with_capacity(0);
        while reader.read_exact(&mut buf).is_ok() {
            for i in buf {
                image_buffer.push(i);
            }
        }
```

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 34.3s
Improvement: 3.3s**
    

### 8. Function new_internal() optimization (get rid of if-else judgement in recursive calls, get rid of branches)

In function new_internal() there are two if statements and can be moved to function new() which calls new_internal():
```rust=
    if triangles.len() == 0 {
        return BVHNode::Leaf {
            bounding_box: BoundingBox::EMPTY,
            triangles,
        };
    }
    if triangles.len() < 30 {
        return BVHNode::Leaf {
            bounding_box,
            triangles,
        };
    }
```
Function new() before optimization:
```rust=
pub fn new(triangles: Vec<Arc<Triangle>>) -> Self {
    debug!("Creating new KD Tree with {} triangles", triangles.len());

    let bb = BoundingBox::from_triangles(triangles.iter().cloned());

    Self::new_internal(triangles, bb, 0)
}
```

Function new() after optimization:
```rust=
pub fn new(triangles: Vec<Arc<Triangle>>) -> Self {
    debug!("Creating new KD Tree with {} triangles", triangles.len());

    let bb = BoundingBox::from_triangles(triangles.iter().cloned());
    if triangles.len() == 0 {
        return BVHNode::Leaf {
            bounding_box: BoundingBox::EMPTY,
            triangles,
        };
    }

    if triangles.len() < 30 {
        return BVHNode::Leaf {
            bounding_box: bb,
            triangles,
        };
    }

    Self::new_internal(triangles, bb, 0)
}
```
Moving the if-statement out of the function avoid branch predictions during recursive calls.

Graph size: 500 * 500 
Sample rate: 200
**Benchmark after optimization: 23.2s
Improvement: 11.1s**
    
    
### 9. Optimization of random generator
The folloing flame graph shows generating random numbers spends a lot of time. So we changed the RNG type from OsRng to thread_rng().
Before optimization:
 ![](https://i.imgur.com/RVdLxF2.png)
 
After optimization:
 ![](https://i.imgur.com/BR7f71u.png)
The time wasted by generator random numbers reduces a lot

    
**Benchmark after optimization: 16.7s
Improvement: 6.5s**
    
  
## III. Benchmark with criterion
### 1.Benchmark of set_at function

Because set_at() function has less and simpler input parameters compared with other improved funuctions, and it shows the best improvement of whole program, so it is chosen to be benchmarked with criterion. 

The benchmark result is shown in the picture below:

After running "cargo bench" several times, time fixes around 51.553 ms.
![](https://i.imgur.com/ajmYhNq.png)

After optimizing the set_at function, it shows an improvement with 51.422ms. This is only about rendering a picture with size 10 * 10. The improvements can be much more signifcant when a picture with larger size is rendered.
![](https://i.imgur.com/f7PQMau.png)

Graph size: 25 * 25
Sample rate: 25

### 2.Benchmark of set_at function
Benchmark of optimization 1:
![](https://i.imgur.com/DAKdSyt.png)

Benchmark of optimization 2:
![](https://i.imgur.com/dcxAdwf.png)

Benchmark of optimization 3:
![](https://i.imgur.com/MxCGZnQ.png)

Benchmark of optimization 4:
![](https://i.imgur.com/X8xp636.png)

Benchmark of optimization 5:
![](https://i.imgur.com/I2dqbND.png)

Benchmark of optimization 6:
![](https://i.imgur.com/JHh7YaU.png)

Benchmark of optimization 7:
![](https://i.imgur.com/PRmt4QV.png)

Benchmark of optimization 8:
![](https://i.imgur.com/Y7bEqEi.png)

Benchmark of optimization 9:
![](https://i.imgur.com/4tOOywh.png)

Graph size: 25 * 25
Sample rate: 25
