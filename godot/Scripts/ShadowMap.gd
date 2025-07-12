extends TileMapLayer

var cells_alpha = {}

func _ready() -> void:
	$"../../Character".moved.connect(process_movement)

func _tile_data_runtime_update(coords: Vector2i, tile_data: TileData) -> void:
	tile_data.modulate.a = cells_alpha.get(coords, 1.0)

func _use_tile_data_runtime_update(coords: Vector2i) -> bool:
	return cells_alpha.has(coords)

func process_movement() -> void:
	pass
	#$"..".

func _process(delta: float) -> void:
	#var xy = position + dir * TILE_SIZE - Vector2(8, 8)
	#var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	#var blocked = $"../MyRustMap".is_blocked(rounded_xy.x, rounded_xy.y)
	pass
	#var cell
	#cells_alpha[cell] = cells_alpha.get(cell, 1.0) - 0.2
	#notify_runtime_tile_data_update()
