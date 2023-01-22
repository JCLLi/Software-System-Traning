# Textual modeling decisions for FMS

## FSM model for a 1-plane system
The following graph shows the FSM model for a 1-plane system. Except for the original state block after entry, there are three other state blocks.

The "Low" and "High" state blocks correspond to the basic state of the "low dose x-ray pedal" and "high dose x-ray pedal" are pressed. The value of "acctivateLowVideo" will be raised to the value of "startLowVideo" when the pedal is pressed and will be raised to the value of "stopLowVideo" when the pedal is released. "deactivate" will also be raised. The same thing will happen to the value of "acctivateHighVideo".

For the particular situation: the high-dose pedal is pressed when the low dose has already been pressed. We decided to set an intermediate state between "Low" and "High" called "High, low pressed". When the system is in this state, if the high dose pedal is released, it will go back to the state "Low". Conversely, when the low dose pedal is released, the system will go directly to the "High" state.


![](https://i.imgur.com/0ZqDjZw.png)


## FSM model for a 2-plane system
The second graph is a little more complex than the first one because 2-plane system has more pedals. As described in the system description, each low-dose pedal has its state when it is pressed. When one of the low pedals is pressed, the system will keep on that projection state and will not be affected by other low-dose pedals unless it is released. For high dose pedal states, they are almost as same as the 2-plane system. But a problem ensues: when a high-dose pedal and a low-dose pedal are pressed at the same time, and the low-dose pedal is released, which state system should go? "Low in pedal1" or "Low in pedal2"? In order to avoid confusing the system, we decided to add a state flag "current_low_pedal" to show which low dose pedal is pressed at the moment. In our design, 1 means low dose pedal 1 (front projection), 2 means low dose pedal 2 (lateral projection), and 3 means low dose pedal 3 (biplane projection).
![](https://i.imgur.com/ISfyKxi.png)

When it comes to the projection toggling, another two value is added called "current_pedal" and "next_pedal". Every time when the system receives a toggle request, these two values change in a RR order to choose every project. And the toggle operations should be able to run parallelly with low or high-dose pedals operations. So we decided to use an orthogonal state block to guarantee they can be run at the same time.

The initial values of different events are 0. Users can set them by themselves. As shown in the following figure, different events in the command class are set to different values. And the figure shows the state of low dose pedal 1 is pressed, and high dose pedal is pressed meanwhile. The current high dose projection is 1. When users click different commands, they can see the changes in event values, state values, or flags in the controller class.

![](https://i.imgur.com/t5o26bx.png)
