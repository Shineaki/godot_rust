extends Node2D

func _ready() -> void:
	$MyRustMap.generate_floors()
	$MyRustMap.generate_walls()
	var spawn_point = $MyRustMap.get_spawn_point()
	$Character.position = spawn_point
	$MyRustMap.init_shadows()
	
	var xy = spawn_point - Vector2(8, 8)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
	#$MyRustMap.update_minimap(rounded_xy)
	$Character.moved.connect(process_movement)

func _process(delta: float) -> void:
	print(delta)
	pass

func process_movement():
	var xy = $Character.position - Vector2(8, 8)
	print("process_movement ", xy)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
	#$MyRustMap.update_minimap(rounded_xy)ddddddddddddddddd
