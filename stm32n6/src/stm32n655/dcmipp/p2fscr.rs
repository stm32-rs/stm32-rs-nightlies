///Register `P2FSCR` reader
pub type R = crate::R<P2FSCRrs>;
///Register `P2FSCR` writer
pub type W = crate::W<P2FSCRrs>;
///Field `DTIDA` reader - Data type ID
pub type DTIDA_R = crate::FieldReader;
///Field `DTIDA` writer - Data type ID
pub type DTIDA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `VC` reader - Flow selection mode
pub type VC_R = crate::FieldReader;
///Field `VC` writer - Flow selection mode
pub type VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FDTF` reader - Force data type format
pub type FDTF_R = crate::FieldReader;
///Field `FDTF` writer - Force data type format
pub type FDTF_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `FDTFEN` reader - Force data type format enable
pub type FDTFEN_R = crate::BitReader;
///Field `FDTFEN` writer - Force data type format enable
pub type FDTFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIPEN` reader - Activation of PipeN
pub type PIPEN_R = crate::BitReader;
///Field `PIPEN` writer - Activation of PipeN
pub type PIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Data type ID
    #[inline(always)]
    pub fn dtida(&self) -> DTIDA_R {
        DTIDA_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 19:20 - Flow selection mode
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 24:29 - Force data type format
    #[inline(always)]
    pub fn fdtf(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Force data type format enable
    #[inline(always)]
    pub fn fdtfen(&self) -> FDTFEN_R {
        FDTFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Activation of PipeN
    #[inline(always)]
    pub fn pipen(&self) -> PIPEN_R {
        PIPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2FSCR")
            .field("dtida", &self.dtida())
            .field("vc", &self.vc())
            .field("fdtf", &self.fdtf())
            .field("fdtfen", &self.fdtfen())
            .field("pipen", &self.pipen())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Data type ID
    #[inline(always)]
    pub fn dtida(&mut self) -> DTIDA_W<'_, P2FSCRrs> {
        DTIDA_W::new(self, 0)
    }
    ///Bits 19:20 - Flow selection mode
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W<'_, P2FSCRrs> {
        VC_W::new(self, 19)
    }
    ///Bits 24:29 - Force data type format
    #[inline(always)]
    pub fn fdtf(&mut self) -> FDTF_W<'_, P2FSCRrs> {
        FDTF_W::new(self, 24)
    }
    ///Bit 30 - Force data type format enable
    #[inline(always)]
    pub fn fdtfen(&mut self) -> FDTFEN_W<'_, P2FSCRrs> {
        FDTFEN_W::new(self, 30)
    }
    ///Bit 31 - Activation of PipeN
    #[inline(always)]
    pub fn pipen(&mut self) -> PIPEN_W<'_, P2FSCRrs> {
        PIPEN_W::new(self, 31)
    }
}
/**DCMIPP Pipe2 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p2fscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2fscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P2FSCR)*/
pub struct P2FSCRrs;
impl crate::RegisterSpec for P2FSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2fscr::R`](R) reader structure
impl crate::Readable for P2FSCRrs {}
///`write(|w| ..)` method takes [`p2fscr::W`](W) writer structure
impl crate::Writable for P2FSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2FSCR to value 0
impl crate::Resettable for P2FSCRrs {}
