#[doc = "Reader of register DCMI_DR"]
pub type R = crate::R<u32, super::DCMI_DR>;
#[doc = "Reader of field `Byte0`"]
pub type BYTE0_R = crate::R<u8, u8>;
#[doc = "Reader of field `Byte1`"]
pub type BYTE1_R = crate::R<u8, u8>;
#[doc = "Reader of field `Byte2`"]
pub type BYTE2_R = crate::R<u8, u8>;
#[doc = "Reader of field `Byte3`"]
pub type BYTE3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Byte0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
