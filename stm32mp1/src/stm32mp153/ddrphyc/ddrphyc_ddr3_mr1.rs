#[doc = "Register `DDRPHYC_DDR3_MR1` reader"]
pub type R = crate::R<DDRPHYC_DDR3_MR1rs>;
#[doc = "Register `DDRPHYC_DDR3_MR1` writer"]
pub type W = crate::W<DDRPHYC_DDR3_MR1rs>;
#[doc = "Field `DE` reader - DE"]
pub type DE_R = crate::BitReader;
#[doc = "Field `DE` writer - DE"]
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIC0` reader - DIC0"]
pub type DIC0_R = crate::BitReader;
#[doc = "Field `DIC0` writer - DIC0"]
pub type DIC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTT0` reader - RTT0"]
pub type RTT0_R = crate::BitReader;
#[doc = "Field `RTT0` writer - RTT0"]
pub type RTT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AL` reader - AL"]
pub type AL_R = crate::FieldReader;
#[doc = "Field `AL` writer - AL"]
pub type AL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIC1` reader - DIC1"]
pub type DIC1_R = crate::BitReader;
#[doc = "Field `DIC1` writer - DIC1"]
pub type DIC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTT1` reader - RTT1"]
pub type RTT1_R = crate::BitReader;
#[doc = "Field `RTT1` writer - RTT1"]
pub type RTT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL` reader - LEVEL"]
pub type LEVEL_R = crate::BitReader;
#[doc = "Field `LEVEL` writer - LEVEL"]
pub type LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTT2` reader - RTT2"]
pub type RTT2_R = crate::BitReader;
#[doc = "Field `RTT2` writer - RTT2"]
pub type RTT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDQS` reader - TDQS"]
pub type TDQS_R = crate::BitReader;
#[doc = "Field `TDQS` writer - TDQS"]
pub type TDQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QOFF` reader - QOFF"]
pub type QOFF_R = crate::BitReader;
#[doc = "Field `QOFF` writer - QOFF"]
pub type QOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&self) -> DIC0_R {
        DIC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&self) -> RTT0_R {
        RTT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&self) -> DIC1_R {
        DIC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&self) -> RTT1_R {
        RTT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&self) -> RTT2_R {
        RTT2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&self) -> TDQS_R {
        TDQS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&self) -> QOFF_R {
        QOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<DDRPHYC_DDR3_MR1rs> {
        DE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    #[must_use]
    pub fn dic0(&mut self) -> DIC0_W<DDRPHYC_DDR3_MR1rs> {
        DIC0_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    #[must_use]
    pub fn rtt0(&mut self) -> RTT0_W<DDRPHYC_DDR3_MR1rs> {
        RTT0_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AL_W<DDRPHYC_DDR3_MR1rs> {
        AL_W::new(self, 3)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    #[must_use]
    pub fn dic1(&mut self) -> DIC1_W<DDRPHYC_DDR3_MR1rs> {
        DIC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    #[must_use]
    pub fn rtt1(&mut self) -> RTT1_W<DDRPHYC_DDR3_MR1rs> {
        RTT1_W::new(self, 6)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<DDRPHYC_DDR3_MR1rs> {
        LEVEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    #[must_use]
    pub fn rtt2(&mut self) -> RTT2_W<DDRPHYC_DDR3_MR1rs> {
        RTT2_W::new(self, 9)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    #[must_use]
    pub fn tdqs(&mut self) -> TDQS_W<DDRPHYC_DDR3_MR1rs> {
        TDQS_W::new(self, 11)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    #[must_use]
    pub fn qoff(&mut self) -> QOFF_W<DDRPHYC_DDR3_MR1rs> {
        QOFF_W::new(self, 12)
    }
}
#[doc = "DDRPHYC MR1 register for DDR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ddr3_mr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DDR3_MR1rs;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR1rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ddrphyc_ddr3_mr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ddr3_mr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR1 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR1rs {
    const RESET_VALUE: u16 = 0;
}
