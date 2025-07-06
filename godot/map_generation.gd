extends Node2D

func _ready() -> void:
	$MyRustMap.generate_floors()
	$MyRustMap.generate_walls()
	var spawn_point = $MyRustMap.get_spawn_point()
	$Character.position = spawn_point
	$MyRustMap.init_shadows()

func _process(delta: float) -> void:
	var xy = $Character.position - Vector2(8, 8)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
