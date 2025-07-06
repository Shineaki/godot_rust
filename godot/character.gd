extends CharacterBody2D


func _input(event):
	if event.is_action_pressed("Attack"):
		attack()

func attack():
	$Hands/RightHandPosition/AnimationPlayer.play("Attack")

@export var speed = 100

func get_input():
	var input_direction = Input.get_vector("Move_Left", "Move_Right", "Move_Up", "Move_Down")
	velocity = input_direction * speed

func _process(delta: float) -> void:
	var mouse_pos = get_global_mouse_position()
	if $AnimatedSprite2D.flip_h != (mouse_pos.x < position.x):
		$Hands.scale.x *= -1
		$AnimatedSprite2D.flip_h = mouse_pos.x < position.x
	print(position)
	print(position.x/16, " ", position.y/16)

func _physics_process(delta):
	#$Hands.look_at()
	get_input()
	move_and_slide()
