///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `SPRIV` reader - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access.
pub type SPRIV_R = crate::BitReader;
///Field `SPRIV` writer - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access.
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPRIV` reader - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.
pub type NSPRIV_R = crate::BitReader;
///Field `NSPRIV` writer - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access.
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("spriv", &self.spriv())
            .field("nspriv", &self.nspriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access.
    #[inline(always)]
    pub fn spriv(&mut self) -> SPRIV_W<PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    ///Bit 1 - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.
    #[inline(always)]
    pub fn nspriv(&mut self) -> NSPRIV_W<PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
/**RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
