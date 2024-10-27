using UnrealBuildTool;

public class Ue5RustPlugin : ModuleRules
{
	public Ue5RustPlugin(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;
		
		PublicIncludePaths.AddRange(new string[] {});
				
		PrivateIncludePaths.AddRange(new string[] {});
			
		PublicDependencyModuleNames.AddRange(
			new string[]
			{
				"Core", "GraphEditor",
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