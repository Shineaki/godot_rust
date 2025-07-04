extends Node2D

func _ready() -> void:
	$MyRustMap.generate_floors()
	$MyRustMap.generate_walls()
	var spawn_point = $MyRustMap.get_spawn_point()
	$Character.position = spawn_point
