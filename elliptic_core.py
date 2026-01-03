class Point:
    def __init__(self, x, y, a, b, p):
        self.x, self.y, self.a, self.b, self.p = x, y, a, b, p

    def __add__(self, other):
        if self.x is None: return other
        if other.x is None: return self
        if self.x == other.x and self.y != other.y: return Point(None, None, self.a, self.b, self.p)
        
        if self == other:
            l = (3 * self.x**2 + self.a) * pow(2 * self.y, -1, self.p)
        else:
            l = (other.y - self.y) * pow(other.x - self.x, -1, self.p)
        
        x3 = (l**2 - self.x - other.x) % self.p
        y3 = (l * (self.x - x3) - self.y) % self.p
        return Point(x3, y3, self.a, self.b, self.p)
      
