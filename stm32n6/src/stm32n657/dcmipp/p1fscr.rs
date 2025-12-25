///Register `P1FSCR` reader
pub type R = crate::R<P1FSCRrs>;
///Register `P1FSCR` writer
pub type W = crate::W<P1FSCRrs>;
///Field `DTIDA` reader - Data type selection ID A
pub type DTIDA_R = crate::FieldReader;
///Field `DTIDA` writer - Data type selection ID A
pub type DTIDA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DTIDB` reader - Data type selection ID B
pub type DTIDB_R = crate::FieldReader;
///Field `DTIDB` writer - Data type selection ID B
pub type DTIDB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DTMODE` reader - Flow selection mode
pub type DTMODE_R = crate::FieldReader;
///Field `DTMODE` writer - Flow selection mode
pub type DTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PIPEDIFF` reader - Differentiates Pipe2 from Pipe1
pub type PIPEDIFF_R = crate::BitReader;
///Field `PIPEDIFF` writer - Differentiates Pipe2 from Pipe1
pub type PIPEDIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VC` reader - Flow selection mode
pub type VC_R = crate::FieldReader;
///Field `VC` writer - Flow selection mode
pub type VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FDTF` reader - Force Datatype format.
pub type FDTF_R = crate::FieldReader;
///Field `FDTF` writer - Force Datatype format.
pub type FDTF_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `FDTFEN` reader - Force Datatype format enable.
pub type FDTFEN_R = crate::BitReader;
///Field `FDTFEN` writer - Force Datatype format enable.
pub type FDTFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIPEN` reader - Activation of PipeN
pub type PIPEN_R = crate::BitReader;
///Field `PIPEN` writer - Activation of PipeN
pub type PIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Data type selection ID A
    #[inline(always)]
    pub fn dtida(&self) -> DTIDA_R {
        DTIDA_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Data type selection ID B
    #[inline(always)]
    pub fn dtidb(&self) -> DTIDB_R {
        DTIDB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:17 - Flow selection mode
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Differentiates Pipe2 from Pipe1
    #[inline(always)]
    pub fn pipediff(&self) -> PIPEDIFF_R {
        PIPEDIFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - Flow selection mode
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 24:29 - Force Datatype format.
    #[inline(always)]
    pub fn fdtf(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Force Datatype format enable.
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
        f.debug_struct("P1FSCR")
            .field("dtida", &self.dtida())
            .field("dtidb", &self.dtidb())
            .field("dtmode", &self.dtmode())
            .field("pipediff", &self.pipediff())
            .field("vc", &self.vc())
            .field("fdtf", &self.fdtf())
            .field("fdtfen", &self.fdtfen())
            .field("pipen", &self.pipen())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Data type selection ID A
    #[inline(always)]
    pub fn dtida(&mut self) -> DTIDA_W<'_, P1FSCRrs> {
        DTIDA_W::new(self, 0)
    }
    ///Bits 8:13 - Data type selection ID B
    #[inline(always)]
    pub fn dtidb(&mut self) -> DTIDB_W<'_, P1FSCRrs> {
        DTIDB_W::new(self, 8)
    }
    ///Bits 16:17 - Flow selection mode
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<'_, P1FSCRrs> {
        DTMODE_W::new(self, 16)
    }
    ///Bit 18 - Differentiates Pipe2 from Pipe1
    #[inline(always)]
    pub fn pipediff(&mut self) -> PIPEDIFF_W<'_, P1FSCRrs> {
        PIPEDIFF_W::new(self, 18)
    }
    ///Bits 19:20 - Flow selection mode
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W<'_, P1FSCRrs> {
        VC_W::new(self, 19)
    }
    ///Bits 24:29 - Force Datatype format.
    #[inline(always)]
    pub fn fdtf(&mut self) -> FDTF_W<'_, P1FSCRrs> {
        FDTF_W::new(self, 24)
    }
    ///Bit 30 - Force Datatype format enable.
    #[inline(always)]
    pub fn fdtfen(&mut self) -> FDTFEN_W<'_, P1FSCRrs> {
        FDTFEN_W::new(self, 30)
    }
    ///Bit 31 - Activation of PipeN
    #[inline(always)]
    pub fn pipen(&mut self) -> PIPEN_W<'_, P1FSCRrs> {
        PIPEN_W::new(self, 31)
    }
}
/**DCMIPP Pipe1 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p1fscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1fscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1FSCR)*/
pub struct P1FSCRrs;
impl crate::RegisterSpec for P1FSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1fscr::R`](R) reader structure
impl crate::Readable for P1FSCRrs {}
///`write(|w| ..)` method takes [`p1fscr::W`](W) writer structure
impl crate::Writable for P1FSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1FSCR to value 0
impl crate::Resettable for P1FSCRrs {}
