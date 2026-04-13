///Register `PRIFCFGR` reader
pub type R = crate::R<PRIFCFGRrs>;
///Register `PRIFCFGR` writer
pub type W = crate::W<PRIFCFGRrs>;
///Field `SPRIV` reader - Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN=1).
pub type SPRIV_R = crate::BitReader;
///Field `SPRIV` writer - Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN=1).
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPRIV` reader - Privileged protection for non-secure registers
pub type NSPRIV_R = crate::BitReader;
///Field `NSPRIV` writer - Privileged protection for non-secure registers
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN=1).
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Privileged protection for non-secure registers
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIFCFGR")
            .field("spriv", &self.spriv())
            .field("nspriv", &self.nspriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN=1).
    #[inline(always)]
    pub fn spriv(&mut self) -> SPRIV_W<'_, PRIFCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    ///Bit 1 - Privileged protection for non-secure registers
    #[inline(always)]
    pub fn nspriv(&mut self) -> NSPRIV_W<'_, PRIFCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
/**FLASH privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`prifcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prifcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:PRIFCFGR)*/
pub struct PRIFCFGRrs;
impl crate::RegisterSpec for PRIFCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`prifcfgr::R`](R) reader structure
impl crate::Readable for PRIFCFGRrs {}
///`write(|w| ..)` method takes [`prifcfgr::W`](W) writer structure
impl crate::Writable for PRIFCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIFCFGR to value 0
impl crate::Resettable for PRIFCFGRrs {}
