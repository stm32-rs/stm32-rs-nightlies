///Register `GADPCTL` reader
pub type R = crate::R<GADPCTLrs>;
///Register `GADPCTL` writer
pub type W = crate::W<GADPCTLrs>;
///Field `PRBDSCHG` reader - Probe discharge
pub type PRBDSCHG_R = crate::FieldReader;
///Field `PRBDSCHG` writer - Probe discharge
pub type PRBDSCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRBDELTA` reader - Probe delta
pub type PRBDELTA_R = crate::FieldReader;
///Field `PRBDELTA` writer - Probe delta
pub type PRBDELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRBPER` reader - Probe period
pub type PRBPER_R = crate::FieldReader;
///Field `PRBPER` writer - Probe period
pub type PRBPER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTIM` reader - Ramp time
pub type RTIM_R = crate::FieldReader<u16>;
///Field `ENAPRB` reader - Enable probe
pub type ENAPRB_R = crate::BitReader;
///Field `ENAPRB` writer - Enable probe
pub type ENAPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENASNS` reader - Enable sense
pub type ENASNS_R = crate::BitReader;
///Field `ENASNS` writer - Enable sense
pub type ENASNS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPRST` reader - ADP reset
pub type ADPRST_R = crate::BitReader;
///Field `ADPEN` reader - ADP enable
pub type ADPEN_R = crate::BitReader;
///Field `ADPEN` writer - ADP enable
pub type ADPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPPRBIF` reader - ADP probe interrupt flag
pub type ADPPRBIF_R = crate::BitReader;
///Field `ADPPRBIF` writer - ADP probe interrupt flag
pub type ADPPRBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPSNSIF` reader - ADP sense interrupt flag
pub type ADPSNSIF_R = crate::BitReader;
///Field `ADPSNSIF` writer - ADP sense interrupt flag
pub type ADPSNSIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPTOIF` reader - ADP timeout interrupt flag
pub type ADPTOIF_R = crate::BitReader;
///Field `ADPTOIF` writer - ADP timeout interrupt flag
pub type ADPTOIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPPRBIM` reader - ADP probe interrupt mask
pub type ADPPRBIM_R = crate::BitReader;
///Field `ADPPRBIM` writer - ADP probe interrupt mask
pub type ADPPRBIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPSNSIM` reader - ADP sense interrupt mask
pub type ADPSNSIM_R = crate::BitReader;
///Field `ADPSNSIM` writer - ADP sense interrupt mask
pub type ADPSNSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADPTOIM` reader - ADP timeout interrupt mask
pub type ADPTOIM_R = crate::BitReader;
///Field `ADPTOIM` writer - ADP timeout interrupt mask
pub type ADPTOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AR` reader - Access request
pub type AR_R = crate::FieldReader;
///Field `AR` writer - Access request
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Probe discharge
    #[inline(always)]
    pub fn prbdschg(&self) -> PRBDSCHG_R {
        PRBDSCHG_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Probe delta
    #[inline(always)]
    pub fn prbdelta(&self) -> PRBDELTA_R {
        PRBDELTA_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Probe period
    #[inline(always)]
    pub fn prbper(&self) -> PRBPER_R {
        PRBPER_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:16 - Ramp time
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    ///Bit 17 - Enable probe
    #[inline(always)]
    pub fn enaprb(&self) -> ENAPRB_R {
        ENAPRB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable sense
    #[inline(always)]
    pub fn enasns(&self) -> ENASNS_R {
        ENASNS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ADP reset
    #[inline(always)]
    pub fn adprst(&self) -> ADPRST_R {
        ADPRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ADP enable
    #[inline(always)]
    pub fn adpen(&self) -> ADPEN_R {
        ADPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADP probe interrupt flag
    #[inline(always)]
    pub fn adpprbif(&self) -> ADPPRBIF_R {
        ADPPRBIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADP sense interrupt flag
    #[inline(always)]
    pub fn adpsnsif(&self) -> ADPSNSIF_R {
        ADPSNSIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADP timeout interrupt flag
    #[inline(always)]
    pub fn adptoif(&self) -> ADPTOIF_R {
        ADPTOIF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADP probe interrupt mask
    #[inline(always)]
    pub fn adpprbim(&self) -> ADPPRBIM_R {
        ADPPRBIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ADP sense interrupt mask
    #[inline(always)]
    pub fn adpsnsim(&self) -> ADPSNSIM_R {
        ADPSNSIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ADP timeout interrupt mask
    #[inline(always)]
    pub fn adptoim(&self) -> ADPTOIM_R {
        ADPTOIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - Access request
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GADPCTL")
            .field("prbdschg", &self.prbdschg())
            .field("prbdelta", &self.prbdelta())
            .field("prbper", &self.prbper())
            .field("rtim", &self.rtim())
            .field("enaprb", &self.enaprb())
            .field("enasns", &self.enasns())
            .field("adprst", &self.adprst())
            .field("adpen", &self.adpen())
            .field("adpprbif", &self.adpprbif())
            .field("adpsnsif", &self.adpsnsif())
            .field("adptoif", &self.adptoif())
            .field("adpprbim", &self.adpprbim())
            .field("adpsnsim", &self.adpsnsim())
            .field("adptoim", &self.adptoim())
            .field("ar", &self.ar())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Probe discharge
    #[inline(always)]
    pub fn prbdschg(&mut self) -> PRBDSCHG_W<'_, GADPCTLrs> {
        PRBDSCHG_W::new(self, 0)
    }
    ///Bits 2:3 - Probe delta
    #[inline(always)]
    pub fn prbdelta(&mut self) -> PRBDELTA_W<'_, GADPCTLrs> {
        PRBDELTA_W::new(self, 2)
    }
    ///Bits 4:5 - Probe period
    #[inline(always)]
    pub fn prbper(&mut self) -> PRBPER_W<'_, GADPCTLrs> {
        PRBPER_W::new(self, 4)
    }
    ///Bit 17 - Enable probe
    #[inline(always)]
    pub fn enaprb(&mut self) -> ENAPRB_W<'_, GADPCTLrs> {
        ENAPRB_W::new(self, 17)
    }
    ///Bit 18 - Enable sense
    #[inline(always)]
    pub fn enasns(&mut self) -> ENASNS_W<'_, GADPCTLrs> {
        ENASNS_W::new(self, 18)
    }
    ///Bit 20 - ADP enable
    #[inline(always)]
    pub fn adpen(&mut self) -> ADPEN_W<'_, GADPCTLrs> {
        ADPEN_W::new(self, 20)
    }
    ///Bit 21 - ADP probe interrupt flag
    #[inline(always)]
    pub fn adpprbif(&mut self) -> ADPPRBIF_W<'_, GADPCTLrs> {
        ADPPRBIF_W::new(self, 21)
    }
    ///Bit 22 - ADP sense interrupt flag
    #[inline(always)]
    pub fn adpsnsif(&mut self) -> ADPSNSIF_W<'_, GADPCTLrs> {
        ADPSNSIF_W::new(self, 22)
    }
    ///Bit 23 - ADP timeout interrupt flag
    #[inline(always)]
    pub fn adptoif(&mut self) -> ADPTOIF_W<'_, GADPCTLrs> {
        ADPTOIF_W::new(self, 23)
    }
    ///Bit 24 - ADP probe interrupt mask
    #[inline(always)]
    pub fn adpprbim(&mut self) -> ADPPRBIM_W<'_, GADPCTLrs> {
        ADPPRBIM_W::new(self, 24)
    }
    ///Bit 25 - ADP sense interrupt mask
    #[inline(always)]
    pub fn adpsnsim(&mut self) -> ADPSNSIM_W<'_, GADPCTLrs> {
        ADPSNSIM_W::new(self, 25)
    }
    ///Bit 26 - ADP timeout interrupt mask
    #[inline(always)]
    pub fn adptoim(&mut self) -> ADPTOIM_W<'_, GADPCTLrs> {
        ADPTOIM_W::new(self, 26)
    }
    ///Bits 27:28 - Access request
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<'_, GADPCTLrs> {
        AR_W::new(self, 27)
    }
}
/**OTG ADP timer, control and status register

You can [`read`](crate::Reg::read) this register and get [`gadpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gadpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#OTG_FS_GLOBAL:GADPCTL)*/
pub struct GADPCTLrs;
impl crate::RegisterSpec for GADPCTLrs {
    type Ux = u32;
}
///`read()` method returns [`gadpctl::R`](R) reader structure
impl crate::Readable for GADPCTLrs {}
///`write(|w| ..)` method takes [`gadpctl::W`](W) writer structure
impl crate::Writable for GADPCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GADPCTL to value 0x0200_0400
impl crate::Resettable for GADPCTLrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
