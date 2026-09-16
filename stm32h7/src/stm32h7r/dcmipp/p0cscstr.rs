///Register `P0CSCSTR` reader
pub type R = crate::R<P0CSCSTRrs>;
///Field `HSTART` reader - Current horizontal start, from 0 to 4094 words wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `VSTART` reader - Current vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Current horizontal start, from 0 to 4094 words wide
    #[inline(always)]
    pub fn hstart(&self) -> HSTART_R {
        HSTART_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Current vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&self) -> VSTART_R {
        VSTART_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0CSCSTR")
            .field("hstart", &self.hstart())
            .field("vstart", &self.vstart())
            .finish()
    }
}
/**DCMIPP Pipe0 current stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0cscstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CSCSTR)*/
pub struct P0CSCSTRrs;
impl crate::RegisterSpec for P0CSCSTRrs {
    type Ux = u32;
}
///`read()` method returns [`p0cscstr::R`](R) reader structure
impl crate::Readable for P0CSCSTRrs {}
///`reset()` method sets P0CSCSTR to value 0
impl crate::Resettable for P0CSCSTRrs {}
