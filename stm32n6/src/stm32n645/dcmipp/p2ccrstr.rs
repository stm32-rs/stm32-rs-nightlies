///Register `P2CCRSTR` reader
pub type R = crate::R<P2CCRSTRrs>;
///Field `HSTART` reader - Current horizontal start, from 0 to 4094 pixels wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `VSTART` reader - Current vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Current horizontal start, from 0 to 4094 pixels wide
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
        f.debug_struct("P2CCRSTR")
            .field("hstart", &self.hstart())
            .field("vstart", &self.vstart())
            .finish()
    }
}
/**DCMIPP Pipex current crop window start register

You can [`read`](crate::Reg::read) this register and get [`p2ccrstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P2CCRSTR)*/
pub struct P2CCRSTRrs;
impl crate::RegisterSpec for P2CCRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`p2ccrstr::R`](R) reader structure
impl crate::Readable for P2CCRSTRrs {}
///`reset()` method sets P2CCRSTR to value 0
impl crate::Resettable for P2CCRSTRrs {}
