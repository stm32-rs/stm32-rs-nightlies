///Register `CSQISR` reader
pub type R = crate::R<CSQISRrs>;
///Register `CSQISR` writer
pub type W = crate::W<CSQISRrs>;
///Field `TCF` reader - TCF
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - TCF
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCF` reader - SCF
pub type SCF_R = crate::BitReader;
///Field `SCF` writer - SCF
pub type SCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEF` reader - SEF
pub type SEF_R = crate::BitReader;
///Field `SEF` writer - SEF
pub type SEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUEF` reader - SUEF
pub type SUEF_R = crate::BitReader;
///Field `SUEF` writer - SUEF
pub type SUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDTCF` reader - CMDTCF
pub type CMDTCF_R = crate::BitReader;
///Field `CMDTCF` writer - CMDTCF
pub type CMDTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TCF
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCF
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEF
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SUEF
    #[inline(always)]
    pub fn suef(&self) -> SUEF_R {
        SUEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CMDTCF
    #[inline(always)]
    pub fn cmdtcf(&self) -> CMDTCF_R {
        CMDTCF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQISR")
            .field("tcf", &self.tcf())
            .field("scf", &self.scf())
            .field("sef", &self.sef())
            .field("suef", &self.suef())
            .field("cmdtcf", &self.cmdtcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - TCF
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<'_, CSQISRrs> {
        TCF_W::new(self, 0)
    }
    ///Bit 1 - SCF
    #[inline(always)]
    pub fn scf(&mut self) -> SCF_W<'_, CSQISRrs> {
        SCF_W::new(self, 1)
    }
    ///Bit 2 - SEF
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W<'_, CSQISRrs> {
        SEF_W::new(self, 2)
    }
    ///Bit 3 - SUEF
    #[inline(always)]
    pub fn suef(&mut self) -> SUEF_W<'_, CSQISRrs> {
        SUEF_W::new(self, 3)
    }
    ///Bit 4 - CMDTCF
    #[inline(always)]
    pub fn cmdtcf(&mut self) -> CMDTCF_W<'_, CSQISRrs> {
        CMDTCF_W::new(self, 4)
    }
}
/**FMC NAND Command Sequencer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`csqisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQISR)*/
pub struct CSQISRrs;
impl crate::RegisterSpec for CSQISRrs {
    type Ux = u32;
}
///`read()` method returns [`csqisr::R`](R) reader structure
impl crate::Readable for CSQISRrs {}
///`write(|w| ..)` method takes [`csqisr::W`](W) writer structure
impl crate::Writable for CSQISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQISR to value 0
impl crate::Resettable for CSQISRrs {}
