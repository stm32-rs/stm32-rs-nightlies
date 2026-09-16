///Register `DTPR1` reader
pub type R = crate::R<DTPR1rs>;
///Register `DTPR1` writer
pub type W = crate::W<DTPR1rs>;
///Field `TAOND` reader - TAOND
pub type TAOND_R = crate::FieldReader;
///Field `TAOND` writer - TAOND
pub type TAOND_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRTW` reader - TRTW
pub type TRTW_R = crate::BitReader;
///Field `TRTW` writer - TRTW
pub type TRTW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFAW` reader - TFAW
pub type TFAW_R = crate::FieldReader;
///Field `TFAW` writer - TFAW
pub type TFAW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TMOD` reader - TMOD
pub type TMOD_R = crate::FieldReader;
///Field `TMOD` writer - TMOD
pub type TMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRTODT` reader - TRTODT
pub type TRTODT_R = crate::BitReader;
///Field `TRTODT` writer - TRTODT
pub type TRTODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRFC` reader - TRFC
pub type TRFC_R = crate::FieldReader;
///Field `TRFC` writer - TRFC
pub type TRFC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TDQSCKMIN` reader - TDQSCKMIN
pub type TDQSCKMIN_R = crate::FieldReader;
///Field `TDQSCKMIN` writer - TDQSCKMIN
pub type TDQSCKMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TDQSCKMAX` reader - TDQSCKMAX
pub type TDQSCKMAX_R = crate::FieldReader;
///Field `TDQSCKMAX` writer - TDQSCKMAX
pub type TDQSCKMAX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - TAOND
    #[inline(always)]
    pub fn taond(&self) -> TAOND_R {
        TAOND_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - TRTW
    #[inline(always)]
    pub fn trtw(&self) -> TRTW_R {
        TRTW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:8 - TFAW
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    ///Bits 9:10 - TMOD
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - TRTODT
    #[inline(always)]
    pub fn trtodt(&self) -> TRTODT_R {
        TRTODT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:23 - TRFC
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - TDQSCKMIN
    #[inline(always)]
    pub fn tdqsckmin(&self) -> TDQSCKMIN_R {
        TDQSCKMIN_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - TDQSCKMAX
    #[inline(always)]
    pub fn tdqsckmax(&self) -> TDQSCKMAX_R {
        TDQSCKMAX_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTPR1")
            .field("taond", &self.taond())
            .field("trtw", &self.trtw())
            .field("tfaw", &self.tfaw())
            .field("tmod", &self.tmod())
            .field("trtodt", &self.trtodt())
            .field("trfc", &self.trfc())
            .field("tdqsckmin", &self.tdqsckmin())
            .field("tdqsckmax", &self.tdqsckmax())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TAOND
    #[inline(always)]
    pub fn taond(&mut self) -> TAOND_W<'_, DTPR1rs> {
        TAOND_W::new(self, 0)
    }
    ///Bit 2 - TRTW
    #[inline(always)]
    pub fn trtw(&mut self) -> TRTW_W<'_, DTPR1rs> {
        TRTW_W::new(self, 2)
    }
    ///Bits 3:8 - TFAW
    #[inline(always)]
    pub fn tfaw(&mut self) -> TFAW_W<'_, DTPR1rs> {
        TFAW_W::new(self, 3)
    }
    ///Bits 9:10 - TMOD
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W<'_, DTPR1rs> {
        TMOD_W::new(self, 9)
    }
    ///Bit 11 - TRTODT
    #[inline(always)]
    pub fn trtodt(&mut self) -> TRTODT_W<'_, DTPR1rs> {
        TRTODT_W::new(self, 11)
    }
    ///Bits 16:23 - TRFC
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W<'_, DTPR1rs> {
        TRFC_W::new(self, 16)
    }
    ///Bits 24:26 - TDQSCKMIN
    #[inline(always)]
    pub fn tdqsckmin(&mut self) -> TDQSCKMIN_W<'_, DTPR1rs> {
        TDQSCKMIN_W::new(self, 24)
    }
    ///Bits 27:29 - TDQSCKMAX
    #[inline(always)]
    pub fn tdqsckmax(&mut self) -> TDQSCKMAX_W<'_, DTPR1rs> {
        TDQSCKMAX_W::new(self, 27)
    }
}
/**DDRPHYC DTP register 1

You can [`read`](crate::Reg::read) this register and get [`dtpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DTPR1)*/
pub struct DTPR1rs;
impl crate::RegisterSpec for DTPR1rs {
    type Ux = u32;
}
///`read()` method returns [`dtpr1::R`](R) reader structure
impl crate::Readable for DTPR1rs {}
///`write(|w| ..)` method takes [`dtpr1::W`](W) writer structure
impl crate::Writable for DTPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTPR1 to value 0x0a03_0090
impl crate::Resettable for DTPR1rs {
    const RESET_VALUE: u32 = 0x0a03_0090;
}
