///Register `CCU_CWD` reader
pub type R = crate::R<CCU_CWDrs>;
///Register `CCU_CWD` writer
pub type W = crate::W<CCU_CWDrs>;
///Field `WDC` reader - WDC
pub type WDC_R = crate::FieldReader<u16>;
///Field `WDC` writer - WDC
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `WDV` reader - Watchdog value
pub type WDV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Watchdog value
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCU_CWD")
            .field("wdc", &self.wdc())
            .field("wdv", &self.wdv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDC
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<'_, CCU_CWDrs> {
        WDC_W::new(self, 0)
    }
}
/**FDCAN data bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`ccu_cwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_cwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FDCAN1:CCU_CWD)*/
pub struct CCU_CWDrs;
impl crate::RegisterSpec for CCU_CWDrs {
    type Ux = u32;
}
///`read()` method returns [`ccu_cwd::R`](R) reader structure
impl crate::Readable for CCU_CWDrs {}
///`write(|w| ..)` method takes [`ccu_cwd::W`](W) writer structure
impl crate::Writable for CCU_CWDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCU_CWD to value 0x0a33
impl crate::Resettable for CCU_CWDrs {
    const RESET_VALUE: u32 = 0x0a33;
}
