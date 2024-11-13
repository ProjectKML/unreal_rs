#pragma once

#include "GameFramework/GameModeBase.h"
#include "RustGameModeBase.generated.h"

UCLASS()
class RUSTPLUGIN_API ARustGameModeBase : public AGameModeBase {
	GENERATED_BODY()
private:
	bool Initialized = false;
public:
	ARustGameModeBase();
	~ARustGameModeBase();

	virtual void StartPlay();
	virtual void Tick(float Dt);
};