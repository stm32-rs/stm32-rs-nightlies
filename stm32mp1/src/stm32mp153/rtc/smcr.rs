///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `ALRADPROT` reader - ALRADPROT
pub type ALRADPROT_R = crate::BitReader;
///Field `ALRADPROT` writer - ALRADPROT
pub type ALRADPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBDPROT` reader - ALRBDPROT
pub type ALRBDPROT_R = crate::BitReader;
///Field `ALRBDPROT` writer - ALRBDPROT
pub type ALRBDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTDPROT` reader - WUTDPROT
pub type WUTDPROT_R = crate::BitReader;
///Field `WUTDPROT` writer - WUTDPROT
pub type WUTDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSDPROT` reader - TSDPROT
pub type TSDPROT_R = crate::BitReader;
///Field `TSDPROT` writer - TSDPROT
pub type TSDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALDPROT` reader - CALDPROT
pub type CALDPROT_R = crate::BitReader;
///Field `CALDPROT` writer - CALDPROT
pub type CALDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITDPROT` reader - INITDPROT
pub type INITDPROT_R = crate::BitReader;
///Field `INITDPROT` writer - INITDPROT
pub type INITDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DECPROT` reader - DECPROT
pub type DECPROT_R = crate::BitReader;
///Field `DECPROT` writer - DECPROT
pub type DECPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ALRADPROT
    #[inline(always)]
    pub fn alradprot(&self) -> ALRADPROT_R {
        ALRADPROT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ALRBDPROT
    #[inline(always)]
    pub fn alrbdprot(&self) -> ALRBDPROT_R {
        ALRBDPROT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUTDPROT
    #[inline(always)]
    pub fn wutdprot(&self) -> WUTDPROT_R {
        WUTDPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSDPROT
    #[inline(always)]
    pub fn tsdprot(&self) -> TSDPROT_R {
        TSDPROT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - CALDPROT
    #[inline(always)]
    pub fn caldprot(&self) -> CALDPROT_R {
        CALDPROT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - INITDPROT
    #[inline(always)]
    pub fn initdprot(&self) -> INITDPROT_R {
        INITDPROT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DECPROT
    #[inline(always)]
    pub fn decprot(&self) -> DECPROT_R {
        DECPROT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("alradprot", &self.alradprot())
            .field("alrbdprot", &self.alrbdprot())
            .field("wutdprot", &self.wutdprot())
            .field("tsdprot", &self.tsdprot())
            .field("caldprot", &self.caldprot())
            .field("initdprot", &self.initdprot())
            .field("decprot", &self.decprot())
            .finish()
    }
}
impl W {
    ///Bit 0 - ALRADPROT
    #[inline(always)]
    pub fn alradprot(&mut self) -> ALRADPROT_W<'_, SMCRrs> {
        ALRADPROT_W::new(self, 0)
    }
    ///Bit 1 - ALRBDPROT
    #[inline(always)]
    pub fn alrbdprot(&mut self) -> ALRBDPROT_W<'_, SMCRrs> {
        ALRBDPROT_W::new(self, 1)
    }
    ///Bit 2 - WUTDPROT
    #[inline(always)]
    pub fn wutdprot(&mut self) -> WUTDPROT_W<'_, SMCRrs> {
        WUTDPROT_W::new(self, 2)
    }
    ///Bit 3 - TSDPROT
    #[inline(always)]
    pub fn tsdprot(&mut self) -> TSDPROT_W<'_, SMCRrs> {
        TSDPROT_W::new(self, 3)
    }
    ///Bit 13 - CALDPROT
    #[inline(always)]
    pub fn caldprot(&mut self) -> CALDPROT_W<'_, SMCRrs> {
        CALDPROT_W::new(self, 13)
    }
    ///Bit 14 - INITDPROT
    #[inline(always)]
    pub fn initdprot(&mut self) -> INITDPROT_W<'_, SMCRrs> {
        INITDPROT_W::new(self, 14)
    }
    ///Bit 15 - DECPROT
    #[inline(always)]
    pub fn decprot(&mut self) -> DECPROT_W<'_, SMCRrs> {
        DECPROT_W::new(self, 15)
    }
}
/**This register can be written only when the APB access is secure.

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0xe00f
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0xe00f;
}
