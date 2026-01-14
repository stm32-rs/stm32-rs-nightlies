///Register `UR15` reader
pub type R = crate::R<UR15rs>;
///Register `UR15` writer
pub type W = crate::W<UR15rs>;
///Field `D2STPRST` reader - D2 Stop Reset
pub type D2STPRST_R = crate::BitReader;
///Field `D2STPRST` writer - D2 Stop Reset
pub type D2STPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode
pub type FZIWDGSTB_R = crate::BitReader;
impl R {
    ///Bit 0 - D2 Stop Reset
    #[inline(always)]
    pub fn d2stprst(&self) -> D2STPRST_R {
        D2STPRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Freeze independent watchdog in Standby mode
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR15")
            .field("d2stprst", &self.d2stprst())
            .field("fziwdgstb", &self.fziwdgstb())
            .finish()
    }
}
impl W {
    ///Bit 0 - D2 Stop Reset
    #[inline(always)]
    pub fn d2stprst(&mut self) -> D2STPRST_W<'_, UR15rs> {
        D2STPRST_W::new(self, 0)
    }
}
/**SYSCFG user register 15

You can [`read`](crate::Reg::read) this register and get [`ur15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#SYSCFG:UR15)*/
pub struct UR15rs;
impl crate::RegisterSpec for UR15rs {
    type Ux = u32;
}
///`read()` method returns [`ur15::R`](R) reader structure
impl crate::Readable for UR15rs {}
///`write(|w| ..)` method takes [`ur15::W`](W) writer structure
impl crate::Writable for UR15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR15 to value 0
impl crate::Resettable for UR15rs {}
