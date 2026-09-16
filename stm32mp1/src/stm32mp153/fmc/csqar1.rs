///Register `CSQAR1` reader
pub type R = crate::R<CSQAR1rs>;
///Register `CSQAR1` writer
pub type W = crate::W<CSQAR1rs>;
///Field `ADDC1` reader - ADDC1
pub type ADDC1_R = crate::FieldReader;
///Field `ADDC1` writer - ADDC1
pub type ADDC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADDC2` reader - ADDC2
pub type ADDC2_R = crate::FieldReader;
///Field `ADDC2` writer - ADDC2
pub type ADDC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADDC3` reader - ADDC3
pub type ADDC3_R = crate::FieldReader;
///Field `ADDC3` writer - ADDC3
pub type ADDC3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADDC4` reader - ADDC4
pub type ADDC4_R = crate::FieldReader;
///Field `ADDC4` writer - ADDC4
pub type ADDC4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ADDC1
    #[inline(always)]
    pub fn addc1(&self) -> ADDC1_R {
        ADDC1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ADDC2
    #[inline(always)]
    pub fn addc2(&self) -> ADDC2_R {
        ADDC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ADDC3
    #[inline(always)]
    pub fn addc3(&self) -> ADDC3_R {
        ADDC3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ADDC4
    #[inline(always)]
    pub fn addc4(&self) -> ADDC4_R {
        ADDC4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQAR1")
            .field("addc1", &self.addc1())
            .field("addc2", &self.addc2())
            .field("addc3", &self.addc3())
            .field("addc4", &self.addc4())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ADDC1
    #[inline(always)]
    pub fn addc1(&mut self) -> ADDC1_W<'_, CSQAR1rs> {
        ADDC1_W::new(self, 0)
    }
    ///Bits 8:15 - ADDC2
    #[inline(always)]
    pub fn addc2(&mut self) -> ADDC2_W<'_, CSQAR1rs> {
        ADDC2_W::new(self, 8)
    }
    ///Bits 16:23 - ADDC3
    #[inline(always)]
    pub fn addc3(&mut self) -> ADDC3_W<'_, CSQAR1rs> {
        ADDC3_W::new(self, 16)
    }
    ///Bits 24:31 - ADDC4
    #[inline(always)]
    pub fn addc4(&mut self) -> ADDC4_W<'_, CSQAR1rs> {
        ADDC4_W::new(self, 24)
    }
}
/**This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.

You can [`read`](crate::Reg::read) this register and get [`csqar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQAR1)*/
pub struct CSQAR1rs;
impl crate::RegisterSpec for CSQAR1rs {
    type Ux = u32;
}
///`read()` method returns [`csqar1::R`](R) reader structure
impl crate::Readable for CSQAR1rs {}
///`write(|w| ..)` method takes [`csqar1::W`](W) writer structure
impl crate::Writable for CSQAR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQAR1 to value 0
impl crate::Resettable for CSQAR1rs {}
