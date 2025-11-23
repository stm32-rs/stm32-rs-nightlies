///Register `DBGR` reader
pub type R = crate::R<DBGRrs>;
///Register `DBGR` writer
pub type W = crate::W<DBGRrs>;
///Field `DBGHSIOFF` reader - used for debug or test 0: No effect (default) 1: HSI forced off.
pub type DBGHSIOFF_R = crate::BitReader;
///Field `DBGHSIOFF` writer - used for debug or test 0: No effect (default) 1: HSI forced off.
pub type DBGHSIOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGBYPHSI` reader - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE.
pub type DBGBYPHSI_R = crate::BitReader;
///Field `DBGBYPHSI` writer - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE.
pub type DBGBYPHSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGXOEXT` reader - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used.
pub type DBGXOEXT_R = crate::BitReader;
///Field `DBGXOEXT` writer - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used.
pub type DBGXOEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCEXO48MREADY` reader - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1
pub type FORCEXO48MREADY_R = crate::BitReader;
///Field `FORCEXO48MREADY` writer - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1
pub type FORCEXO48MREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 19 - used for debug or test 0: No effect (default) 1: HSI forced off.
    #[inline(always)]
    pub fn dbghsioff(&self) -> DBGHSIOFF_R {
        DBGHSIOFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE.
    #[inline(always)]
    pub fn dbgbyphsi(&self) -> DBGBYPHSI_R {
        DBGBYPHSI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used.
    #[inline(always)]
    pub fn dbgxoext(&self) -> DBGXOEXT_R {
        DBGXOEXT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1
    #[inline(always)]
    pub fn forcexo48mready(&self) -> FORCEXO48MREADY_R {
        FORCEXO48MREADY_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR")
            .field("dbghsioff", &self.dbghsioff())
            .field("dbgbyphsi", &self.dbgbyphsi())
            .field("dbgxoext", &self.dbgxoext())
            .field("forcexo48mready", &self.forcexo48mready())
            .finish()
    }
}
impl W {
    ///Bit 19 - used for debug or test 0: No effect (default) 1: HSI forced off.
    #[inline(always)]
    pub fn dbghsioff(&mut self) -> DBGHSIOFF_W<'_, DBGRrs> {
        DBGHSIOFF_W::new(self, 19)
    }
    ///Bit 20 - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE.
    #[inline(always)]
    pub fn dbgbyphsi(&mut self) -> DBGBYPHSI_W<'_, DBGRrs> {
        DBGBYPHSI_W::new(self, 20)
    }
    ///Bit 21 - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used.
    #[inline(always)]
    pub fn dbgxoext(&mut self) -> DBGXOEXT_W<'_, DBGRrs> {
        DBGXOEXT_W::new(self, 21)
    }
    ///Bit 22 - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1
    #[inline(always)]
    pub fn forcexo48mready(&mut self) -> FORCEXO48MREADY_W<'_, DBGRrs> {
        FORCEXO48MREADY_W::new(self, 22)
    }
}
/**DBGR register

You can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:DBGR)*/
pub struct DBGRrs;
impl crate::RegisterSpec for DBGRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgr::R`](R) reader structure
impl crate::Readable for DBGRrs {}
///`write(|w| ..)` method takes [`dbgr::W`](W) writer structure
impl crate::Writable for DBGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGR to value 0
impl crate::Resettable for DBGRrs {}
