# MechSim

A 3D robot visualizer, physics simulator, and robot code verification tool.

# Importing robots

In order to properly simulate a robot, a robot must be translated from an existing CAD environment to a manageable state capable of describing the physical characteristics that allow the physics engine to simulate the robot. 

## From Onshape

In the menu bar, go to `File > Import robot from Onshape`. You will then go to the Onshape developer portal and generate API keys. Copy and paste the keys into the popup window in MechSim. These keys allow MechSim to access your Onshape account and generate the MJCF format for MechSim to use. In addition, MechSim needs to know which assembly you want to be exported, so open the assembly your robot is in and copy and paste the Onshape URL from the browser into MechSim. Make sure the document you open is an assembly.

> **IMPORTANT:** **Do not** share your keys with anyone else and do not store them in an insecure location where others can easily find.

This feature is currently using a custom made [API client](https://github.com/mechsimulator/onshape-mjcf-exporter) to translate Onshape assemblies into XML-formatted MJCF models.

## From Fusion 360

A Fusion 360 add-in is planned for the future.

## From any other CAD program

Plugins and tools will need to be made specific for those programs. However, in the meantime, you can export your robot as a `STEP` and import it into Fusion 360 or Onshape. From there you will need to remake all of the joints and ground/fix the assembly.