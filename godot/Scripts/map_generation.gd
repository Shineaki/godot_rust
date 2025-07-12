extends Node2D

const GRID_SIZE = 16
@onready var fog = $FOW
var fogImage = Image.new()
var fogTexture = ImageTexture.new()

var lightImage = Image.new()
var light_offset = Vector2(4, 4)

func _ready() -> void:
	fogImage = Image.create(100, 80, false, Image.FORMAT_RGBAH)
	fogImage.fill(Color.BLACK)

	fog.scale *= GRID_SIZE
	fog.texture = ImageTexture.create_from_image(fogImage)
	
	$MyRustMap.generate_floors()
	$MyRustMap.generate_walls()
	var spawn_point = $MyRustMap.get_spawn_point()
	$Character.position = spawn_point

	var xy = spawn_point - Vector2(8, 8)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
	$Character.moved.connect(process_movement)

func _process(delta: float) -> void:
	print(delta)

func process_movement():
	var xy = $Character.position - Vector2(8, 8)
	print("process_movement ", xy)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
