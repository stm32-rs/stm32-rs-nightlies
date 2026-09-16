///Register `UR9` reader
pub type R = crate::R<UR9rs>;
///Field `WRPN_2` reader - Write protection for flash bank 2
pub type WRPN_2_R = crate::FieldReader;
///Field `PA_BEG_2` reader - Protected area start address for bank 2
pub type PA_BEG_2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:7 - Write protection for flash bank 2
    #[inline(always)]
    pub fn wrpn_2(&self) -> WRPN_2_R {
        WRPN_2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:27 - Protected area start address for bank 2
    #[inline(always)]
    pub fn pa_beg_2(&self) -> PA_BEG_2_R {
        PA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR9")
            .field("wrpn_2", &self.wrpn_2())
            .field("pa_beg_2", &self.pa_beg_2())
            .finish()
    }
}
/**SYSCFG user register 9

You can [`read`](crate::Reg::read) this register and get [`ur9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SYSCFG:UR9)*/
pub struct UR9rs;
impl crate::RegisterSpec for UR9rs {
    type Ux = u32;
}
///`read()` method returns [`ur9::R`](R) reader structure
impl crate::Readable for UR9rs {}
///`reset()` method sets UR9 to value 0
impl crate::Resettable for UR9rs {}
