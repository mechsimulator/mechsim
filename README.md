# MechSim

A 3D robot visualizer, physics simulator, and robot code verification tool.

# Importing robots

In order to properly simulate a robot, a robot must be translated from an existing CAD environment to a manageable state capable of describing the physical characteristics that allow the physics engine to simulate the robot. 

## From OnShape

In the menu bar, go to `File > Import robot from OnShape`. You will then go to the OnShape developer portal and generate API keys. Copy and paste the keys into the popup window in MechSim. These keys allow MechSim to access your OnShape account and generate the MJCF format for MechSim to use. In addition, MechSim needs to know which assembly you want to be exported, so open the assembly your robot is in and copy and paste the OnShape URL from the browser into MechSim. Make sure the document you open is an assembly.

[IMPORTANT: Do not share your keys with anyone else and do not store them in an insecure location where others can easily find.]

## From Fusion 360

[In Progress]

## From Any Other CAD Program

Plugins and tools will need to be made specific for those programs, however, in the meantime, you can export your robot as a `STEP` and import it into Fusion 360 or OnShape. From there you will need to remake all of the joints and ground/fix the assembly.