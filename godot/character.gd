extends Player

const TILE_SIZE = Vector2(16, 16)
var sprite_movement_tween: Tween
var TARGET_POSITION = Vector2(0, 0)

func _ready() -> void:
	pass
func _input(event):
	if event.is_action_pressed("Attack"):
		attack()

func attack():
	$Hands/RightHandPosition/AnimationPlayer.play("Attack")

func move(dir: Vector2):
	print("moving ", global_position, " ", global_position + dir * TILE_SIZE)
	TARGET_POSITION = global_position + dir * TILE_SIZE
	
	if sprite_movement_tween:
		if sprite_movement_tween.is_running():
			return
		sprite_movement_tween.kill()
		
	sprite_movement_tween = create_tween()
	sprite_movement_tween.set_process_mode(Tween.TWEEN_PROCESS_PHYSICS)
	sprite_movement_tween.tween_property(self, "global_position", TARGET_POSITION, 0.15)
	sprite_movement_tween.set_trans(Tween.TRANS_SINE)

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("Move_Left"):
		move(Vector2(-1, 0))
	elif Input.is_action_just_pressed("Move_Right"):
		move(Vector2(1, 0))
	elif Input.is_action_just_pressed("Move_Down"):
		move(Vector2(0, 1))
	elif Input.is_action_just_pressed("Move_Up"):
		move(Vector2(0, -1))
	elif Input.is_action_just_pressed("Move_UpLeft"):
		move(Vector2(-1, -1))
	elif Input.is_action_just_pressed("Move_UpRight"):
		move(Vector2(1, -1))
	elif Input.is_action_just_pressed("Move_DownLeft"):
		move(Vector2(-1, 1))
	elif Input.is_action_just_pressed("Move_DownRight"):
		move(Vector2(1, 1))
		
	var mouse_pos = get_global_mouse_position()
	if $AnimatedSprite2D.flip_h != (mouse_pos.x < position.x):
		$Hands.scale.x *= -1
		$AnimatedSprite2D.flip_h = mouse_pos.x < position.x
#
