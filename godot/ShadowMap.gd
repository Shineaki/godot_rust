extends TileMapLayer

var cells_alpha = {}

func _tile_data_runtime_update(coords: Vector2i, tile_data: TileData) -> void:
	tile_data.modulate.a = cells_alpha.get(coords, 1.0)

func _use_tile_data_runtime_update(coords: Vector2i) -> bool:
	return cells_alpha.has(coords)
func _process(delta: float) -> void:
	
	pass
	#var cell
	#cells_alpha[cell] = cells_alpha.get(cell, 1.0) - 0.2
	#notify_runtime_tile_data_update()
