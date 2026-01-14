///Register `WPTCR` reader
pub type R = crate::R<WPTCRrs>;
///Register `WPTCR` writer
pub type W = crate::W<WPTCRrs>;
///Field `DCYC` reader - Number of dummy cycles
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DHQC` reader - Delay hold quarter cycle
pub type DHQC_R = crate::BitReader;
///Field `DHQC` writer - Delay hold quarter cycle
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSHIFT` reader - Sample shift
pub type SSHIFT_R = crate::BitReader;
///Field `SSHIFT` writer - Sample shift
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Number of dummy cycles
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Sample shift
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPTCR")
            .field("dcyc", &self.dcyc())
            .field("dhqc", &self.dhqc())
            .field("sshift", &self.sshift())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of dummy cycles
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W<'_, WPTCRrs> {
        DCYC_W::new(self, 0)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&mut self) -> DHQC_W<'_, WPTCRrs> {
        DHQC_W::new(self, 28)
    }
    ///Bit 30 - Sample shift
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<'_, WPTCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
/**XSPI wrap timing configuration register

You can [`read`](crate::Reg::read) this register and get [`wptcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wptcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#XSPI1:WPTCR)*/
pub struct WPTCRrs;
impl crate::RegisterSpec for WPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`wptcr::R`](R) reader structure
impl crate::Readable for WPTCRrs {}
///`write(|w| ..)` method takes [`wptcr::W`](W) writer structure
impl crate::Writable for WPTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPTCR to value 0
impl crate::Resettable for WPTCRrs {}
