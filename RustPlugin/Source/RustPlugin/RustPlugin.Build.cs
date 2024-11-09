// Copyright Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;

public class RustPlugin : ModuleRules
{
	public RustPlugin(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;
		
		PublicIncludePaths.AddRange(new string[] {});
		
		PrivateIncludePaths.AddRange(new string[] {});
		
		PublicDependencyModuleNames.AddRange(
			new string[]
			{
				"Core",
			}
		);
		
		PrivateDependencyModuleNames.AddRange(
			new string[]
			{
				"Projects",
				"InputCore",
				"EditorFramework",
				"UnrealEd",
				"ToolMenus",
				"CoreUObject",
				"Engine",
				"Slate",
				"SlateCore",
				"BlueprintGraph",
				"GraphEditor",
				"KismetWidgets",
				"KismetCompiler",
				"PropertyEditor",
				"EditorWidgets",
				"ClassViewer",
				"EditorStyle",
			}
		);
		
		DynamicallyLoadedModuleNames.AddRange(new string[] {});
	}
}
