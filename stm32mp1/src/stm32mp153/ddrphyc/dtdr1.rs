///Register `DTDR1` reader
pub type R = crate::R<DTDR1rs>;
///Register `DTDR1` writer
pub type W = crate::W<DTDR1rs>;
///Field `DTBYTE4` reader - DTBYTE4
pub type DTBYTE4_R = crate::FieldReader;
///Field `DTBYTE4` writer - DTBYTE4
pub type DTBYTE4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE5` reader - DTBYTE5
pub type DTBYTE5_R = crate::FieldReader;
///Field `DTBYTE5` writer - DTBYTE5
pub type DTBYTE5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE6` reader - DTBYTE6
pub type DTBYTE6_R = crate::FieldReader;
///Field `DTBYTE6` writer - DTBYTE6
pub type DTBYTE6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE7` reader - DTBYTE7
pub type DTBYTE7_R = crate::FieldReader;
///Field `DTBYTE7` writer - DTBYTE7
pub type DTBYTE7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DTBYTE4
    #[inline(always)]
    pub fn dtbyte4(&self) -> DTBYTE4_R {
        DTBYTE4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DTBYTE5
    #[inline(always)]
    pub fn dtbyte5(&self) -> DTBYTE5_R {
        DTBYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DTBYTE6
    #[inline(always)]
    pub fn dtbyte6(&self) -> DTBYTE6_R {
        DTBYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DTBYTE7
    #[inline(always)]
    pub fn dtbyte7(&self) -> DTBYTE7_R {
        DTBYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTDR1")
            .field("dtbyte4", &self.dtbyte4())
            .field("dtbyte5", &self.dtbyte5())
            .field("dtbyte6", &self.dtbyte6())
            .field("dtbyte7", &self.dtbyte7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DTBYTE4
    #[inline(always)]
    pub fn dtbyte4(&mut self) -> DTBYTE4_W<'_, DTDR1rs> {
        DTBYTE4_W::new(self, 0)
    }
    ///Bits 8:15 - DTBYTE5
    #[inline(always)]
    pub fn dtbyte5(&mut self) -> DTBYTE5_W<'_, DTDR1rs> {
        DTBYTE5_W::new(self, 8)
    }
    ///Bits 16:23 - DTBYTE6
    #[inline(always)]
    pub fn dtbyte6(&mut self) -> DTBYTE6_W<'_, DTDR1rs> {
        DTBYTE6_W::new(self, 16)
    }
    ///Bits 24:31 - DTBYTE7
    #[inline(always)]
    pub fn dtbyte7(&mut self) -> DTBYTE7_W<'_, DTDR1rs> {
        DTBYTE7_W::new(self, 24)
    }
}
/**DDRPHYC DTD register 1

You can [`read`](crate::Reg::read) this register and get [`dtdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DTDR1)*/
pub struct DTDR1rs;
impl crate::RegisterSpec for DTDR1rs {
    type Ux = u32;
}
///`read()` method returns [`dtdr1::R`](R) reader structure
impl crate::Readable for DTDR1rs {}
///`write(|w| ..)` method takes [`dtdr1::W`](W) writer structure
impl crate::Writable for DTDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTDR1 to value 0x7788_bb44
impl crate::Resettable for DTDR1rs {
    const RESET_VALUE: u32 = 0x7788_bb44;
}
