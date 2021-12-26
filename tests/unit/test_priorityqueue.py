import unlocked

def test_get_max():
    priority_queue = unlocked.PriorityQueue()
    priority_queue.insert(13)
    priority_queue.insert(25)
    assert priority_queue.get_max() == 25
    assert priority_queue.len() == 1

def test_get_max():
    priority_queue = unlocked.PriorityQueue()
    priority_queue.insert(13)
    priority_queue.insert(25)
    assert priority_queue.get_min() == 13
    assert priority_queue.len() == 1

def test_order():
    priority_queue = unlocked.PriorityQueue()
    priority_queue.insert(3)
    priority_queue.insert(5)
    priority_queue.insert(4)
    priority_queue.insert(2)
    priority_queue.insert(1)
    assert priority_queue.arr == [5,4,3,2,1]
    assert priority_queue.get_max() == 5
    assert priority_queue.len() == 4

    priority_queue = unlocked.PriorityQueue()
    priority_queue.insert(5)
    priority_queue.insert(1)
    priority_queue.insert(999)
    priority_queue.insert(1000)
    priority_queue.insert(3)
    priority_queue.insert(20)
    assert priority_queue.arr == [1000, 999,20,5,3,1]

    priority_queue = unlocked.PriorityQueue()
    priority_queue.insert(3)
    priority_queue.insert(3)
    priority_queue.insert(3)
    priority_queue.insert(3)
    assert priority_queue.arr == [3,3,3,3]

def test_no_element():
    priority_queue = unlocked.PriorityQueue()
    
    try:
        priority_queue.get_max() 
    except ValueError as e:
        assert str(e) == "Priority Queue is empty"

    try:
        priority_queue.get_min() 
    except ValueError as e:
        assert str(e) == "Priority Queue is empty"


