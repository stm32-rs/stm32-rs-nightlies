///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Register `HWCFGR` writer
pub type W = crate::W<HWCFGRrs>;
///Field `WINDOW` reader - Support of Window function
pub type WINDOW_R = crate::FieldReader;
///Field `WINDOW` writer - Support of Window function
pub type WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PR_DEFAULT` reader - Prescaler default value
pub type PR_DEFAULT_R = crate::FieldReader;
///Field `PR_DEFAULT` writer - Prescaler default value
pub type PR_DEFAULT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Support of Window function
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Prescaler default value
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("window", &self.window())
            .field("pr_default", &self.pr_default())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Support of Window function
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W<'_, HWCFGRrs> {
        WINDOW_W::new(self, 0)
    }
    ///Bits 4:7 - Prescaler default value
    #[inline(always)]
    pub fn pr_default(&mut self) -> PR_DEFAULT_W<'_, HWCFGRrs> {
        PR_DEFAULT_W::new(self, 4)
    }
}
/**hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#IWDG:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`write(|w| ..)` method takes [`hwcfgr::W`](W) writer structure
impl crate::Writable for HWCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR to value 0x71
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x71;
}
