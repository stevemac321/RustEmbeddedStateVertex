
pub struct PriorityQueue {
    heap: [u32; 32],  // Fixed-size array with 32 elements
    length: usize,
    heap_size: usize,
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            heap: [0; 32], // Start with an empty heap
            length: 32,     // No elements initially
            heap_size: 0,  // No elements in heap
        }
    }
    // Constructor with the array initialized from input
    pub fn from_array(input: [u32; 32]) -> Self {
        let mut heap = [0; 32];
        
        // Copy input elements into heap
        for i in 0..input.len() {
            heap[i] = input[i];
        }

        PriorityQueue {
            heap,
            length: input.len(),
            heap_size: input.len(),
        }
    }

    // Internal helper: Returns the index of the parent of element at index i
    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    // Internal helper: Returns the index of the left child of element at index i
    fn left(&self, i: usize) -> usize {
        2 * i + 1
    }

    // Internal helper: Returns the index of the right child of element at index i
    fn right(&self, i: usize) -> usize {
        2 * i + 2
    }

    // Internal method: Maintains the max-heap property starting from index i
    pub fn max_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut largest = i;

        if l < self.heap_size && self.heap[l] > self.heap[i] {
            largest = l;
        }
        if r < self.heap_size && self.heap[r] > self.heap[largest] {
            largest = r;
        }
        if largest != i {
            self.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    // Public API: Builds a max-heap from the current array
    pub fn build_max_heap(&mut self) {
        //self.heap_size = self.length;  // Set heap size to length
        for i in (0..self.length / 2).rev() {
            self.max_heapify(i);
        }
    }
    pub fn build_min_heap(&mut self) {
    }

    // Public API: Insert an integer into the heap
    pub fn insert(&mut self, value: u32) {
        if self.heap_size == self.length {
            return;  // Heap is full
        }
        self.heap[self.heap_size] = value;
        self.heap_size += 1;
        self.build_max_heap();
    }
    pub fn pop(&mut self) -> Option<u32> {
        if self.heap_size == 0 {
            return None;  // No elements left to pop
        }

        let top_value = self.heap[0];
        self.heap[0] = self.heap[self.heap_size - 1];  // Move last element to top
        self.heap_size -= 1;
        self.max_heapify(0);  // Restore heap property
        
        Some(top_value)
    }
    // Swap two elements in the heap
    pub fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.heap[i];
        self.heap[i] = self.heap[j];
        self.heap[j] = tmp;
    }
    pub fn print_heap(&self) {
        for (i, &value) in self.heap.iter().enumerate() {
            //hprintln!("Index {}: {}", i, value);
            let value = i;
        }
    }
    pub fn is_empty(&self) -> bool {
        self.heap_size == 0
    }
}