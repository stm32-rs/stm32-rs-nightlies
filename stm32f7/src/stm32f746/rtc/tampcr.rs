///Register `TAMPCR` reader
pub type R = crate::R<TAMPCRrs>;
///Register `TAMPCR` writer
pub type W = crate::W<TAMPCRrs>;
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - Tamper 1 detection enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for tamper 1
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for tamper 1
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2E` reader - Tamper 2 detection enable
pub type TAMP2E_R = crate::BitReader;
///Field `TAMP2E` writer - Tamper 2 detection enable
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2TRG` reader - Active level for tamper 2
pub type TAMP2TRG_R = crate::BitReader;
///Field `TAMP2TRG` writer - Active level for tamper 2
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3E` reader - Tamper 3 detection enable
pub type TAMP3E_R = crate::BitReader;
///Field `TAMP3E` writer - Tamper 3 detection enable
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3TRG` reader - Active level for tamper 3
pub type TAMP3TRG_R = crate::BitReader;
///Field `TAMP3TRG` writer - Active level for tamper 3
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader;
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader;
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAMPFLT` reader - Tamper filter count
pub type TAMPFLT_R = crate::FieldReader;
///Field `TAMPFLT` writer - Tamper filter count
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPRCH` reader - Tamper precharge duration
pub type TAMPPRCH_R = crate::FieldReader;
///Field `TAMPPRCH` writer - Tamper precharge duration
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPUDIS` reader - TAMPER pull-up disable
pub type TAMPPUDIS_R = crate::BitReader;
///Field `TAMPPUDIS` writer - TAMPER pull-up disable
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1IE` reader - Tamper 1 interrupt enable
pub type TAMP1IE_R = crate::BitReader;
///Field `TAMP1IE` writer - Tamper 1 interrupt enable
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1NOERASE` reader - Tamper 1 no erase
pub type TAMP1NOERASE_R = crate::BitReader;
///Field `TAMP1NOERASE` writer - Tamper 1 no erase
pub type TAMP1NOERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1MF` reader - Tamper 1 mask flag
pub type TAMP1MF_R = crate::BitReader;
///Field `TAMP1MF` writer - Tamper 1 mask flag
pub type TAMP1MF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2IE` reader - Tamper 2 interrupt enable
pub type TAMP2IE_R = crate::BitReader;
///Field `TAMP2IE` writer - Tamper 2 interrupt enable
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2NOERASE` reader - Tamper 2 no erase
pub type TAMP2NOERASE_R = crate::BitReader;
///Field `TAMP2NOERASE` writer - Tamper 2 no erase
pub type TAMP2NOERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2MF` reader - Tamper 2 mask flag
pub type TAMP2MF_R = crate::BitReader;
///Field `TAMP2MF` writer - Tamper 2 mask flag
pub type TAMP2MF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3IE` reader - Tamper 3 interrupt enable
pub type TAMP3IE_R = crate::BitReader;
///Field `TAMP3IE` writer - Tamper 3 interrupt enable
pub type TAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3NOERASE` reader - Tamper 3 no erase
pub type TAMP3NOERASE_R = crate::BitReader;
///Field `TAMP3NOERASE` writer - Tamper 3 no erase
pub type TAMP3NOERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3MF` reader - Tamper 3 mask flag
pub type TAMP3MF_R = crate::BitReader;
///Field `TAMP3MF` writer - Tamper 3 mask flag
pub type TAMP3MF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tamper 3 detection enable
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Active level for tamper 3
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tamper 1 mask flag
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Tamper 2 mask flag
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Tamper 3 mask flag
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMPCR")
            .field("tamp1e", &self.tamp1e())
            .field("tamp1trg", &self.tamp1trg())
            .field("tampie", &self.tampie())
            .field("tamp2e", &self.tamp2e())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp3e", &self.tamp3e())
            .field("tamp3trg", &self.tamp3trg())
            .field("tampts", &self.tampts())
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp1noerase", &self.tamp1noerase())
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2ie", &self.tamp2ie())
            .field("tamp2noerase", &self.tamp2noerase())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp3ie", &self.tamp3ie())
            .field("tamp3noerase", &self.tamp3noerase())
            .field("tamp3mf", &self.tamp3mf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, TAMPCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, TAMPCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, TAMPCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<'_, TAMPCRrs> {
        TAMP2E_W::new(self, 3)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<'_, TAMPCRrs> {
        TAMP2TRG_W::new(self, 4)
    }
    ///Bit 5 - Tamper 3 detection enable
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<'_, TAMPCRrs> {
        TAMP3E_W::new(self, 5)
    }
    ///Bit 6 - Active level for tamper 3
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<'_, TAMPCRrs> {
        TAMP3TRG_W::new(self, 6)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<'_, TAMPCRrs> {
        TAMPTS_W::new(self, 7)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<'_, TAMPCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<'_, TAMPCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<'_, TAMPCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<'_, TAMPCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    ///Bit 16 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, TAMPCRrs> {
        TAMP1IE_W::new(self, 16)
    }
    ///Bit 17 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W<'_, TAMPCRrs> {
        TAMP1NOERASE_W::new(self, 17)
    }
    ///Bit 18 - Tamper 1 mask flag
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W<'_, TAMPCRrs> {
        TAMP1MF_W::new(self, 18)
    }
    ///Bit 19 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<'_, TAMPCRrs> {
        TAMP2IE_W::new(self, 19)
    }
    ///Bit 20 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W<'_, TAMPCRrs> {
        TAMP2NOERASE_W::new(self, 20)
    }
    ///Bit 21 - Tamper 2 mask flag
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W<'_, TAMPCRrs> {
        TAMP2MF_W::new(self, 21)
    }
    ///Bit 22 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<'_, TAMPCRrs> {
        TAMP3IE_W::new(self, 22)
    }
    ///Bit 23 - Tamper 3 no erase
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W<'_, TAMPCRrs> {
        TAMP3NOERASE_W::new(self, 23)
    }
    ///Bit 24 - Tamper 3 mask flag
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W<'_, TAMPCRrs> {
        TAMP3MF_W::new(self, 24)
    }
}
/**tamper configuration register

You can [`read`](crate::Reg::read) this register and get [`tampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#RTC:TAMPCR)*/
pub struct TAMPCRrs;
impl crate::RegisterSpec for TAMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`tampcr::R`](R) reader structure
impl crate::Readable for TAMPCRrs {}
///`write(|w| ..)` method takes [`tampcr::W`](W) writer structure
impl crate::Writable for TAMPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TAMPCR to value 0
impl crate::Resettable for TAMPCRrs {}
