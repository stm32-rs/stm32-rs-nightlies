///Register `PWR_PRIVCFGR` reader
pub type R = crate::R<PWR_PRIVCFGRrs>;
///Register `PWR_PRIVCFGR` writer
pub type W = crate::W<PWR_PRIVCFGRrs>;
///Field `SPRIV` reader - PWR secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access.
pub type SPRIV_R = crate::BitReader;
///Field `SPRIV` writer - PWR secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access.
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPRIV` reader - PWR non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure.
pub type NSPRIV_R = crate::BitReader;
///Field `NSPRIV` writer - PWR non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure.
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PWR secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access.
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PWR non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure.
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_PRIVCFGR")
            .field("spriv", &self.spriv())
            .field("nspriv", &self.nspriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - PWR secure functions privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access.
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<PWR_PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    ///Bit 1 - PWR non-secure functions privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure.
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<PWR_PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
/**PWR privilege control register

You can [`read`](crate::Reg::read) this register and get [`pwr_privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#PWR:PWR_PRIVCFGR)*/
pub struct PWR_PRIVCFGRrs;
impl crate::RegisterSpec for PWR_PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_privcfgr::R`](R) reader structure
impl crate::Readable for PWR_PRIVCFGRrs {}
///`write(|w| ..)` method takes [`pwr_privcfgr::W`](W) writer structure
impl crate::Writable for PWR_PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_PRIVCFGR to value 0
impl crate::Resettable for PWR_PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
