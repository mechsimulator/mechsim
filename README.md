# MechSim

> 🚧 This project is currently under heavy development and is not in a usable state. 🚧

A 3D robot visualizer, physics simulator, and robot code verification tool. _Start iterating and programming your robot from week one._

# Importing robots

In order to properly simulate a robot, a robot must be translated from an existing CAD environment to a manageable state capable of describing the physical characteristics that allow the physics engine to simulate the robot. 

## From Onshape

In the menu bar, go to `File > Import robot from Onshape`. You will then go to the Onshape developer portal and generate API keys. Copy and paste the keys into the popup window in MechSim. These keys allow MechSim to access your Onshape account and generate the MJCF format for MechSim to use. In addition, MechSim needs to know which assembly you want to be exported, so open the assembly your robot is in and copy and paste the Onshape URL from the browser into MechSim. Make sure the document you open is an assembly.

**IMPORTANT:** **Do not** share your keys with anyone else and do not store them in an insecure location where others can easily find.

> This feature is currently using a custom made [API client](https://github.com/mechsimulator/onshape-mjcf-exporter) to translate Onshape assemblies into XML-formatted MJCF models.

## From Fusion 360

A Fusion 360 add-in is planned for the future.

## From any other CAD program

Plugins and tools will need to be made specific for those programs. However, in the meantime, you can export your robot as a `STEP` and import it into Fusion 360 or Onshape. From there you will need to remake all of the joints and ground/fix the assembly.

- - - - 

# Current Plans

- Exporting robots from Onshape and Fusion 360
- Physics engine integration to simulate robot models
- Joint characterization to increase simulation accuracy (Tune joint properties to behave like real bearings, gears, etc.)

# Future Plans

- Robot code integration
- Electrical configuration (CAN, PWM, motors, motor controllers, IMUs, encoders, etc.)
- Mechanical configuration (custom mechanisms)
- Soft body components like belts, cables, and game pieces
- Force heat map to indicate where the most stress is being applied and to predict which parts are more likely to break
- Vision simulation
