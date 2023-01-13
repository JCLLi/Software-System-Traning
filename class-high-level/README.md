# Textual modeling decisions for each PlantUML

**This is important! Please note that the introduction and the textual modeling decisions of each PlantUML graph are delivered below**

## Compenent Diagram
### Introduction
The figure below shows the component diagram of the interventional X-ray systems. The diagram involves no actor, only hardware, software and electrical components, additional interfaces are introduced when necessary. Though the diagram does not shown a clear border between hardware components and software/electrical components, it clearly displays the logic of the system. One branch starting from the database configures the system, while the other branch starting from the padels activates the Xray hardware and streams the generated images on the screen.

![](https://i.imgur.com/Ve9RhOs.png)

### Textual modeling decisions
1. Is it necessary to create a bunch of packages? Like "Hardware" including pedals, tube, detector, etc. And "Software/Electronic" including mapper, logic, controller, etc. It was given a try, however, the result turned out to be that the less packages there are, the clearer and more logical the diagram is. After a discussion with the TAs and the professor, it has been decided to keep the amount of packages small, but arrange the components more logically. Hence, in the final delivered diagram, only the pedals and the hardware related to the generation/detection of Xray is grouped together in a package.
2. Should the patient table be invloved in this diagram? Both situation seems reasonable. Patient table is a necesaary component in the interventional X-ray systems. The whole point of the system is to treat the patient who is lying on the patient table. However, it is also reasonable to not have it included in the diagram due to the fact that the patient table does not influence the functionality of the system. After a heated discussion, the final decision is to include the patient table in the diagram. The reasons are as follows: First, the whole system is meant to be used on the patient lying on the patient table. So, the system defintely would not be powered on when there is no patient. These machines are usually extremely expensive and very power consuming. Hence they would only be operated when necessary, meaning when a patient in need of medical treatment in lying on the table. Second, on the basis of the first point. The Xray the tube outputs and the Xray the detector collects is different. The earlier one does not carry patient's information while the latter one does.
3. Should the detector directly pass on the Xray collected to the Image processor, or should it pass it on to the controller first? The answer to this doubt turns out to be clearly stated in the description of the system. It wrote "...Then it synchronizes their execution using timed pulses (and receives X-ray images from the X-ray detector) until deactivation.". Hence, the detector first passes on the optical information collected to the controller, and then the controller might process the data a bit before forwarding it for image processing.

## Sequence Diagram - Configure
### Introduction
The figure below shows the sequence diagram of the configuration stage of the interventional X-ray systems. The configuration of the system invloves an actor, a database and several components. The actor which refers to the surgeon in this system first checks all the existing settings on a tablet before the surgery starts. All existing settings are saved in a database. The database updates the list in the tablet each time the surgeon requests to. After choosing the most suitable setting for the surgery, the tablet displays the the chosen setting to the surgeon again, and starts sending the corresponding setting to the system configurator. The configurator then processes the setting into several commands and forwards them asynchrnously to required compoents in the system. Each component would reply a message containing the status of the corresponding component. The message basically confirms whether the configuration is successful or not. If not, the tablet could display the error message to the surgeon. Hence the surgery would not start without the expected settings. However, the process of error handling is not displayed in the sequence diagram in detail. It is more invloved during the design phase.

![](https://i.imgur.com/j3YIHxC.png)


### Textual modeling decisions

1. There is a doubt whether the actor "Surgeon" should be activated. The decision has been made that it should not be activated and deactivated. The reason is that "Surgeon" is a human being which has no limitation on whether it is functioning or not.
2. Should the tablet be activated only when accessing it, or throughout the process? The decision is to activate it throughout the process. It is how tablets work in real life. The tablet is activated when the surgeon asks to check settings, and it is deactivated later when the chosen setting by the surgeon is configured. Note that the status of the configuration will also be shown, this is related to question 4.
3. The interaction between the tablet and the database should be simple. There was a thought of letting the Surgeon set up whatever he thinks is required for the surgery, and then check whether the setting exists in the database. However, that seems a bit redundant. Why not display all existing settings to the surgeon and let him choose among them? Hence it is applied. The database only communicates to the tablet once when the surgeon asks to setup settings. It displays all available settings to the surgeon at once (probably in a rather user-friendly way, but this is for further designs in detial when in comes to the interaction). The surgeon could only choose among the existing settings for the surgery. It lowers the risk of surgeon making mistakes when configurating the settings. Anything related to human lives should be dealt with carefully.
4. As for configurator, processor, controller, director and the tube. An issue regarding the feedback aroused while designing. The issue is that should these components have a feedback signal? The signal in this case of configuration contains the information of the status of the components. If the configuration failed, then it sends a "failed" signal, and vice versa. The final decision was made to equip with those signals since everything have a potential risk of failing. The surgeon has to know about the failure of the configuration. Hence he won't start the surgery with misconfigured settings.
5. The messages between configurator, processor, controller, detector and tube are asynchrnous. The decision is made base on whether the order of sending the messages are known or not. In the case of configuration, the sequence of action related to the surgeon is known. The tablet only pulls all settings from the database after the surgeon asks for it. The tablet only forwards the chosen setting to the configurator after the surgeon has decided which is the suitable setting for the current surgery. Moreover, the sequence of the rest of the system is unknown. The configurator could configure the controller and the processor asynchrnously. The same idea applies to the detector and the tube. Hence, only the surgeon-related messages are synchronous and the rest are asynchronous.

## Sequence Diagram - One Pedal
### Introduction
The figure below shows the sequence diagram when only the padel for low-dose video is operated. It is very similar to the one for configuration. The synchrnous and asynchrnous messages follows the same pattern. The differences are: First, a loop that generates the Xray images and displays them to the surgeon is added. Second, the system is based on the assumption that the system has already been configured. Meaning that it is in the middle of a surgery. Third, the system activates and displays images to the surgeon when the padel is stepped on, and deactivates when the padel is released.

![](https://i.imgur.com/30hipKO.png)

### Textual modeling decisions
1. Should the "Patient Table" be activated? This issue is similar to the problem stating whether "Surgeon" should be activated. The decision is made that "Patient Table" should be activated when necessary. Though this seems completely contradictory to what has been decided on the "Surgeon" issue, it is actually not. "Surgeon" is an actor, while the "Patient Table" (inlcuding the lying patient) is a participant in the system. With different patients on the "Patient Table", the optical information the detector collects differs. Another advantage of activating the "Patient Table" is that it could show clearly when the duration of time the Xray beam goes through the patient. Hence the decision is made to activate the "Patient Table" when necessary.
2. Is it necessary to return a message from the screen to the surgeon? The final decision is that both answers are reasonable. The reason why the message to the surgeon is added in the end is simply because in the requirement of the third assignment. It stated that "The diagram should still show that images are displayed on the screen.", and due to the fact that component "Screen" is omitted, the only way to show that the images are displayed on the screen is to return a message "Display Readable Message" to the surgeon.


## Sequence Diagram - Two Pedals
### Introduction
The two figures below show the sequence diagram when two padels are both present in the system. Note that some components are omitted in this scenario to avoid complexity, hence a component "Xray Process & Display Hardware" are created to group all the omitted components together. Different from the two sequence diagrams above, the action of the surgeon now becomes asynchrnous, and the rest of the messages becomes synchrnous. The reasons are that, first, it is unknown when the surgeon would need low or high dose Xray during the surgery. If the asynchrnous messages of the surgeon are known, the rest of the messages are synchrnous.

The reason that there are two figures is because of this requirement "While using low-dose streaming video, surgeons need to be able to temporarily switch to high-dose streaming video, without releasing the low-dose streaming video pedal.". It is believed that the solution figure 1 presents functions well, however, it is not clear how the requirement is handled. In figure 2, an option "if high-dose padel not stepped on" is added. It clearly shows that how the requiremnt is handled. However, it is uncertain if this requirement is meant to be realized in a later phase during the design in detial. Hence, both figures are shown. The only difference between them is the option. 

#### Figure 1

![](https://i.imgur.com/heMpaYU.png)

#### Figure 2

![](https://i.imgur.com/g9kV9R4.png)


### Textual modeling decisions
1. Most of the doubts are solved during making the first two sequence diagrams. The only doubt is also mentioned in the previous sequence diagram. It is decision number 2. The final decision made, and the reason behind it are all mentioned above, hence it is not repeated.
2. Another doubt is the related to the requirement "While using low-dose streaming video, surgeons need to be able to temporarily switch to high-dose streaming video, without releasing the low-dose streaming video pedal.". However, the final decision remains ungiven, hence two figures are presented above.


## Class Diagram - Database
### Introduction

![](https://i.imgur.com/7bUeRZl.png)

### Textual modeling decisions


## Class Diagram - High-level
### Introduction

![](https://i.imgur.com/udXiD8y.png)

### Textual modeling decisions
