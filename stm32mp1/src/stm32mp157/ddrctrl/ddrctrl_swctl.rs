///Register `DDRCTRL_SWCTL` reader
pub type R = crate::R<DDRCTRL_SWCTLrs>;
///Register `DDRCTRL_SWCTL` writer
pub type W = crate::W<DDRCTRL_SWCTLrs>;
///Field `SW_DONE` reader - SW_DONE
pub type SW_DONE_R = crate::BitReader;
///Field `SW_DONE` writer - SW_DONE
pub type SW_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SW_DONE
    #[inline(always)]
    pub fn sw_done(&self) -> SW_DONE_R {
        SW_DONE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_SWCTL")
            .field("sw_done", &self.sw_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - SW_DONE
    #[inline(always)]
    #[must_use]
    pub fn sw_done(&mut self) -> SW_DONE_W<DDRCTRL_SWCTLrs> {
        SW_DONE_W::new(self, 0)
    }
}
/**DDRCTRL software register programming control enable

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_swctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_swctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_SWCTL)*/
pub struct DDRCTRL_SWCTLrs;
impl crate::RegisterSpec for DDRCTRL_SWCTLrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_swctl::R`](R) reader structure
impl crate::Readable for DDRCTRL_SWCTLrs {}
///`write(|w| ..)` method takes [`ddrctrl_swctl::W`](W) writer structure
impl crate::Writable for DDRCTRL_SWCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_SWCTL to value 0x01
impl crate::Resettable for DDRCTRL_SWCTLrs {
    const RESET_VALUE: u32 = 0x01;
}
