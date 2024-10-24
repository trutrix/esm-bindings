use super::RecordHeader;




impl std::fmt::Debug for RecordHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.type_id == b"GRUP" {
            write!(f, "Record {{ Type: {:?}, Size, {:?}, Flags as FourCC: {:?}, Id: {:?} }}", {self.type_id}, {self.size}, {fourcc::FourCC(self.flags.to_le_bytes())}, {self.id})
        } else {
        
            write!(f, "Record {{ Type: {:?}, Size, {:?}, Flags: {:?}, Id: {:?} }}", {self.type_id}, {self.size}, {self.flags}, {self.id})
        }
    }
}