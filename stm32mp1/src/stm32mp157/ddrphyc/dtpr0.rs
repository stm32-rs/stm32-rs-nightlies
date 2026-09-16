///Register `DTPR0` reader
pub type R = crate::R<DTPR0rs>;
///Register `DTPR0` writer
pub type W = crate::W<DTPR0rs>;
///Field `TMRD` reader - TMRD
pub type TMRD_R = crate::FieldReader;
///Field `TMRD` writer - TMRD
pub type TMRD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRTP` reader - TRTP
pub type TRTP_R = crate::FieldReader;
///Field `TRTP` writer - TRTP
pub type TRTP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TWTR` reader - TWTR
pub type TWTR_R = crate::FieldReader;
///Field `TWTR` writer - TWTR
pub type TWTR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRP` reader - TRP
pub type TRP_R = crate::FieldReader;
///Field `TRP` writer - TRP
pub type TRP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRCD` reader - TRCD
pub type TRCD_R = crate::FieldReader;
///Field `TRCD` writer - TRCD
pub type TRCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRAS` reader - TRAS
pub type TRAS_R = crate::FieldReader;
///Field `TRAS` writer - TRAS
pub type TRAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRRD` reader - TRRD
pub type TRRD_R = crate::FieldReader;
///Field `TRRD` writer - TRRD
pub type TRRD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRC` reader - TRC
pub type TRC_R = crate::FieldReader;
///Field `TRC` writer - TRC
pub type TRC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TCCD` reader - TCCD
pub type TCCD_R = crate::BitReader;
///Field `TCCD` writer - TCCD
pub type TCCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - TMRD
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - TRTP
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:7 - TWTR
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:11 - TRP
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - TRCD
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - TRAS
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:24 - TRRD
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bits 25:30 - TRC
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    ///Bit 31 - TCCD
    #[inline(always)]
    pub fn tccd(&self) -> TCCD_R {
        TCCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTPR0")
            .field("tmrd", &self.tmrd())
            .field("trtp", &self.trtp())
            .field("twtr", &self.twtr())
            .field("trp", &self.trp())
            .field("trcd", &self.trcd())
            .field("tras", &self.tras())
            .field("trrd", &self.trrd())
            .field("trc", &self.trc())
            .field("tccd", &self.tccd())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TMRD
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W<'_, DTPR0rs> {
        TMRD_W::new(self, 0)
    }
    ///Bits 2:4 - TRTP
    #[inline(always)]
    pub fn trtp(&mut self) -> TRTP_W<'_, DTPR0rs> {
        TRTP_W::new(self, 2)
    }
    ///Bits 5:7 - TWTR
    #[inline(always)]
    pub fn twtr(&mut self) -> TWTR_W<'_, DTPR0rs> {
        TWTR_W::new(self, 5)
    }
    ///Bits 8:11 - TRP
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W<'_, DTPR0rs> {
        TRP_W::new(self, 8)
    }
    ///Bits 12:15 - TRCD
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W<'_, DTPR0rs> {
        TRCD_W::new(self, 12)
    }
    ///Bits 16:20 - TRAS
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W<'_, DTPR0rs> {
        TRAS_W::new(self, 16)
    }
    ///Bits 21:24 - TRRD
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W<'_, DTPR0rs> {
        TRRD_W::new(self, 21)
    }
    ///Bits 25:30 - TRC
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W<'_, DTPR0rs> {
        TRC_W::new(self, 25)
    }
    ///Bit 31 - TCCD
    #[inline(always)]
    pub fn tccd(&mut self) -> TCCD_W<'_, DTPR0rs> {
        TCCD_W::new(self, 31)
    }
}
/**DDRPHYC DTP register 0

You can [`read`](crate::Reg::read) this register and get [`dtpr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTPR0)*/
pub struct DTPR0rs;
impl crate::RegisterSpec for DTPR0rs {
    type Ux = u32;
}
///`read()` method returns [`dtpr0::R`](R) reader structure
impl crate::Readable for DTPR0rs {}
///`write(|w| ..)` method takes [`dtpr0::W`](W) writer structure
impl crate::Writable for DTPR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTPR0 to value 0x3012_666e
impl crate::Resettable for DTPR0rs {
    const RESET_VALUE: u32 = 0x3012_666e;
}
