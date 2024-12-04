///Register `CACR` reader
pub type R = crate::R<CACRrs>;
///Register `CACR` writer
pub type W = crate::W<CACRrs>;
///Field `SIWT` reader - SIWT
pub type SIWT_R = crate::BitReader;
///Field `SIWT` writer - SIWT
pub type SIWT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCEN` reader - ECCEN
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECCEN
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCEWT` reader - FORCEWT
pub type FORCEWT_R = crate::BitReader;
///Field `FORCEWT` writer - FORCEWT
pub type FORCEWT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SIWT
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FORCEWT
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR")
            .field("siwt", &self.siwt())
            .field("eccen", &self.eccen())
            .field("forcewt", &self.forcewt())
            .finish()
    }
}
impl W {
    ///Bit 0 - SIWT
    #[inline(always)]
    pub fn siwt(&mut self) -> SIWT_W<CACRrs> {
        SIWT_W::new(self, 0)
    }
    ///Bit 1 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<CACRrs> {
        ECCEN_W::new(self, 1)
    }
    ///Bit 2 - FORCEWT
    #[inline(always)]
    pub fn forcewt(&mut self) -> FORCEWT_W<CACRrs> {
        FORCEWT_W::new(self, 2)
    }
}
/**Auxiliary Cache Control register

You can [`read`](crate::Reg::read) this register and get [`cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F7x9.html#AC:CACR)*/
pub struct CACRrs;
impl crate::RegisterSpec for CACRrs {
    type Ux = u32;
}
///`read()` method returns [`cacr::R`](R) reader structure
impl crate::Readable for CACRrs {}
///`write(|w| ..)` method takes [`cacr::W`](W) writer structure
impl crate::Writable for CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACR to value 0
impl crate::Resettable for CACRrs {
    const RESET_VALUE: u32 = 0;
}
