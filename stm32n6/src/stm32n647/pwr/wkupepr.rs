///Register `WKUPEPR` reader
pub type R = crate::R<WKUPEPRrs>;
///Register `WKUPEPR` writer
pub type W = crate::W<WKUPEPRrs>;
///Field `WKUPEN1` reader - Enable WKUP1 pin
pub type WKUPEN1_R = crate::BitReader;
///Field `WKUPEN1` writer - Enable WKUP1 pin
pub type WKUPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN2` reader - Enable WKUP2 pin
pub type WKUPEN2_R = crate::BitReader;
///Field `WKUPEN2` writer - Enable WKUP2 pin
pub type WKUPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN3` reader - Enable WKUP3 pin
pub type WKUPEN3_R = crate::BitReader;
///Field `WKUPEN3` writer - Enable WKUP3 pin
pub type WKUPEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN4` reader - Enable WKUP4 pin
pub type WKUPEN4_R = crate::BitReader;
///Field `WKUPEN4` writer - Enable WKUP4 pin
pub type WKUPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP1` reader - Wake-up polarity bit for WKUP1 pin
pub type WKUPP1_R = crate::BitReader;
///Field `WKUPP1` writer - Wake-up polarity bit for WKUP1 pin
pub type WKUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP2` reader - Wake-up polarity bit for WKUP2 pin
pub type WKUPP2_R = crate::BitReader;
///Field `WKUPP2` writer - Wake-up polarity bit for WKUP2 pin
pub type WKUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP3` reader - Wake-up polarity bit for WKUP3 pin
pub type WKUPP3_R = crate::BitReader;
///Field `WKUPP3` writer - Wake-up polarity bit for WKUP3 pin
pub type WKUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP4` reader - Wake-up polarity bit for WKUP4 pin
pub type WKUPP4_R = crate::BitReader;
///Field `WKUPP4` writer - Wake-up polarity bit for WKUP4 pin
pub type WKUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPUPD1` reader - Wake-up pull configuration for WKUP1 pin
pub type WKUPPUPD1_R = crate::FieldReader;
///Field `WKUPPUPD1` writer - Wake-up pull configuration for WKUP1 pin
pub type WKUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD2` reader - Wake-up pull configuration for WKUP2 pin
pub type WKUPPUPD2_R = crate::FieldReader;
///Field `WKUPPUPD2` writer - Wake-up pull configuration for WKUP2 pin
pub type WKUPPUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD3` reader - Wake-up pull configuration for WKUP3 pin
pub type WKUPPUPD3_R = crate::FieldReader;
///Field `WKUPPUPD3` writer - Wake-up pull configuration for WKUP3 pin
pub type WKUPPUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD4` reader - Wake-up pull configuration for WKUP4 pin
pub type WKUPPUPD4_R = crate::FieldReader;
///Field `WKUPPUPD4` writer - Wake-up pull configuration for WKUP4 pin
pub type WKUPPUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable WKUP1 pin
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable WKUP2 pin
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable WKUP3 pin
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable WKUP4 pin
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Wake-up polarity bit for WKUP1 pin
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wake-up polarity bit for WKUP2 pin
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wake-up polarity bit for WKUP3 pin
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wake-up polarity bit for WKUP4 pin
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:17 - Wake-up pull configuration for WKUP1 pin
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Wake-up pull configuration for WKUP2 pin
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Wake-up pull configuration for WKUP3 pin
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Wake-up pull configuration for WKUP4 pin
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPEPR")
            .field("wkupen1", &self.wkupen1())
            .field("wkupen2", &self.wkupen2())
            .field("wkupen3", &self.wkupen3())
            .field("wkupen4", &self.wkupen4())
            .field("wkupp1", &self.wkupp1())
            .field("wkupp2", &self.wkupp2())
            .field("wkupp3", &self.wkupp3())
            .field("wkupp4", &self.wkupp4())
            .field("wkuppupd1", &self.wkuppupd1())
            .field("wkuppupd2", &self.wkuppupd2())
            .field("wkuppupd3", &self.wkuppupd3())
            .field("wkuppupd4", &self.wkuppupd4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable WKUP1 pin
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<'_, WKUPEPRrs> {
        WKUPEN1_W::new(self, 0)
    }
    ///Bit 1 - Enable WKUP2 pin
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<'_, WKUPEPRrs> {
        WKUPEN2_W::new(self, 1)
    }
    ///Bit 2 - Enable WKUP3 pin
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<'_, WKUPEPRrs> {
        WKUPEN3_W::new(self, 2)
    }
    ///Bit 3 - Enable WKUP4 pin
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<'_, WKUPEPRrs> {
        WKUPEN4_W::new(self, 3)
    }
    ///Bit 8 - Wake-up polarity bit for WKUP1 pin
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W<'_, WKUPEPRrs> {
        WKUPP1_W::new(self, 8)
    }
    ///Bit 9 - Wake-up polarity bit for WKUP2 pin
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W<'_, WKUPEPRrs> {
        WKUPP2_W::new(self, 9)
    }
    ///Bit 10 - Wake-up polarity bit for WKUP3 pin
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W<'_, WKUPEPRrs> {
        WKUPP3_W::new(self, 10)
    }
    ///Bit 11 - Wake-up polarity bit for WKUP4 pin
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W<'_, WKUPEPRrs> {
        WKUPP4_W::new(self, 11)
    }
    ///Bits 16:17 - Wake-up pull configuration for WKUP1 pin
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W<'_, WKUPEPRrs> {
        WKUPPUPD1_W::new(self, 16)
    }
    ///Bits 18:19 - Wake-up pull configuration for WKUP2 pin
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W<'_, WKUPEPRrs> {
        WKUPPUPD2_W::new(self, 18)
    }
    ///Bits 20:21 - Wake-up pull configuration for WKUP3 pin
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W<'_, WKUPEPRrs> {
        WKUPPUPD3_W::new(self, 20)
    }
    ///Bits 22:23 - Wake-up pull configuration for WKUP4 pin
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W<'_, WKUPEPRrs> {
        WKUPPUPD4_W::new(self, 22)
    }
}
/**PWR wake-up enable and polarity register

You can [`read`](crate::Reg::read) this register and get [`wkupepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#PWR:WKUPEPR)*/
pub struct WKUPEPRrs;
impl crate::RegisterSpec for WKUPEPRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupepr::R`](R) reader structure
impl crate::Readable for WKUPEPRrs {}
///`write(|w| ..)` method takes [`wkupepr::W`](W) writer structure
impl crate::Writable for WKUPEPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPEPR to value 0
impl crate::Resettable for WKUPEPRrs {}
