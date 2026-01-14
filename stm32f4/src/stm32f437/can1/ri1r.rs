///Register `RI1R` reader
pub type R = crate::R<RI1Rrs>;
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader;
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader;
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16>;
impl R {
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RI1R")
            .field("stid", &self.stid())
            .field("exid", &self.exid())
            .field("ide", &self.ide())
            .field("rtr", &self.rtr())
            .finish()
    }
}
/**mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`ri1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#CAN1:RI1R)*/
pub struct RI1Rrs;
impl crate::RegisterSpec for RI1Rrs {
    type Ux = u32;
}
///`read()` method returns [`ri1r::R`](R) reader structure
impl crate::Readable for RI1Rrs {}
///`reset()` method sets RI1R to value 0
impl crate::Resettable for RI1Rrs {}
