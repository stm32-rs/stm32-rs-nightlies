///Register `SWCTL` reader
pub type R = crate::R<SWCTLrs>;
///Register `SWCTL` writer
pub type W = crate::W<SWCTLrs>;
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
        f.debug_struct("SWCTL")
            .field("sw_done", &self.sw_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - SW_DONE
    #[inline(always)]
    pub fn sw_done(&mut self) -> SW_DONE_W<'_, SWCTLrs> {
        SW_DONE_W::new(self, 0)
    }
}
/**DDRCTRL software register programming control enable

You can [`read`](crate::Reg::read) this register and get [`swctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SWCTL)*/
pub struct SWCTLrs;
impl crate::RegisterSpec for SWCTLrs {
    type Ux = u32;
}
///`read()` method returns [`swctl::R`](R) reader structure
impl crate::Readable for SWCTLrs {}
///`write(|w| ..)` method takes [`swctl::W`](W) writer structure
impl crate::Writable for SWCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWCTL to value 0x01
impl crate::Resettable for SWCTLrs {
    const RESET_VALUE: u32 = 0x01;
}
