///Register `FPCCR` reader
pub type R = crate::R<FPCCRrs>;
///Register `FPCCR` writer
pub type W = crate::W<FPCCRrs>;
///Field `LSPACT` reader - LSPACT
pub type LSPACT_R = crate::BitReader;
///Field `LSPACT` writer - LSPACT
pub type LSPACT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USER` reader - USER
pub type USER_R = crate::BitReader;
///Field `USER` writer - USER
pub type USER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THREAD` reader - THREAD
pub type THREAD_R = crate::BitReader;
///Field `THREAD` writer - THREAD
pub type THREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFRDY` reader - HFRDY
pub type HFRDY_R = crate::BitReader;
///Field `HFRDY` writer - HFRDY
pub type HFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMRDY` reader - MMRDY
pub type MMRDY_R = crate::BitReader;
///Field `MMRDY` writer - MMRDY
pub type MMRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BFRDY` reader - BFRDY
pub type BFRDY_R = crate::BitReader;
///Field `BFRDY` writer - BFRDY
pub type BFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONRDY` reader - MONRDY
pub type MONRDY_R = crate::BitReader;
///Field `MONRDY` writer - MONRDY
pub type MONRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSPEN` reader - LSPEN
pub type LSPEN_R = crate::BitReader;
///Field `LSPEN` writer - LSPEN
pub type LSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASPEN` reader - ASPEN
pub type ASPEN_R = crate::BitReader;
///Field `ASPEN` writer - ASPEN
pub type ASPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSPACT
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USER
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPCCR")
            .field("lspact", &self.lspact())
            .field("user", &self.user())
            .field("thread", &self.thread())
            .field("hfrdy", &self.hfrdy())
            .field("mmrdy", &self.mmrdy())
            .field("bfrdy", &self.bfrdy())
            .field("monrdy", &self.monrdy())
            .field("lspen", &self.lspen())
            .field("aspen", &self.aspen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSPACT
    #[inline(always)]
    pub fn lspact(&mut self) -> LSPACT_W<FPCCRrs> {
        LSPACT_W::new(self, 0)
    }
    ///Bit 1 - USER
    #[inline(always)]
    pub fn user(&mut self) -> USER_W<FPCCRrs> {
        USER_W::new(self, 1)
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    pub fn thread(&mut self) -> THREAD_W<FPCCRrs> {
        THREAD_W::new(self, 3)
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    pub fn hfrdy(&mut self) -> HFRDY_W<FPCCRrs> {
        HFRDY_W::new(self, 4)
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    pub fn mmrdy(&mut self) -> MMRDY_W<FPCCRrs> {
        MMRDY_W::new(self, 5)
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    pub fn bfrdy(&mut self) -> BFRDY_W<FPCCRrs> {
        BFRDY_W::new(self, 6)
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    pub fn monrdy(&mut self) -> MONRDY_W<FPCCRrs> {
        MONRDY_W::new(self, 8)
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    pub fn lspen(&mut self) -> LSPEN_W<FPCCRrs> {
        LSPEN_W::new(self, 30)
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    pub fn aspen(&mut self) -> ASPEN_W<FPCCRrs> {
        ASPEN_W::new(self, 31)
    }
}
/**Floating-point context control register

You can [`read`](crate::Reg::read) this register and get [`fpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#FPU:FPCCR)*/
pub struct FPCCRrs;
impl crate::RegisterSpec for FPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`fpccr::R`](R) reader structure
impl crate::Readable for FPCCRrs {}
///`write(|w| ..)` method takes [`fpccr::W`](W) writer structure
impl crate::Writable for FPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FPCCR to value 0
impl crate::Resettable for FPCCRrs {
    const RESET_VALUE: u32 = 0;
}
