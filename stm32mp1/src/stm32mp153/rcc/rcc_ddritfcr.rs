#[doc = "Register `RCC_DDRITFCR` reader"]
pub type R = crate::R<RCC_DDRITFCRrs>;
#[doc = "Register `RCC_DDRITFCR` writer"]
pub type W = crate::W<RCC_DDRITFCRrs>;
#[doc = "Field `DDRC1EN` reader - DDRC1EN"]
pub type DDRC1EN_R = crate::BitReader;
#[doc = "Field `DDRC1EN` writer - DDRC1EN"]
pub type DDRC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRC1LPEN` reader - DDRC1LPEN"]
pub type DDRC1LPEN_R = crate::BitReader;
#[doc = "Field `DDRC1LPEN` writer - DDRC1LPEN"]
pub type DDRC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRC2EN` reader - DDRC2EN"]
pub type DDRC2EN_R = crate::BitReader;
#[doc = "Field `DDRC2EN` writer - DDRC2EN"]
pub type DDRC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRC2LPEN` reader - DDRC2LPEN"]
pub type DDRC2LPEN_R = crate::BitReader;
#[doc = "Field `DDRC2LPEN` writer - DDRC2LPEN"]
pub type DDRC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPHYCEN` reader - DDRPHYCEN"]
pub type DDRPHYCEN_R = crate::BitReader;
#[doc = "Field `DDRPHYCEN` writer - DDRPHYCEN"]
pub type DDRPHYCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPHYCLPEN` reader - DDRPHYCLPEN"]
pub type DDRPHYCLPEN_R = crate::BitReader;
#[doc = "Field `DDRPHYCLPEN` writer - DDRPHYCLPEN"]
pub type DDRPHYCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCAPBEN` reader - DDRCAPBEN"]
pub type DDRCAPBEN_R = crate::BitReader;
#[doc = "Field `DDRCAPBEN` writer - DDRCAPBEN"]
pub type DDRCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCAPBLPEN` reader - DDRCAPBLPEN"]
pub type DDRCAPBLPEN_R = crate::BitReader;
#[doc = "Field `DDRCAPBLPEN` writer - DDRCAPBLPEN"]
pub type DDRCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIDCGEN` reader - AXIDCGEN"]
pub type AXIDCGEN_R = crate::BitReader;
#[doc = "Field `AXIDCGEN` writer - AXIDCGEN"]
pub type AXIDCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPHYCAPBEN` reader - DDRPHYCAPBEN"]
pub type DDRPHYCAPBEN_R = crate::BitReader;
#[doc = "Field `DDRPHYCAPBEN` writer - DDRPHYCAPBEN"]
pub type DDRPHYCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPHYCAPBLPEN` reader - DDRPHYCAPBLPEN"]
pub type DDRPHYCAPBLPEN_R = crate::BitReader;
#[doc = "Field `DDRPHYCAPBLPEN` writer - DDRPHYCAPBLPEN"]
pub type DDRPHYCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KERDCG_DLY` reader - KERDCG_DLY"]
pub type KERDCG_DLY_R = crate::FieldReader;
#[doc = "Field `KERDCG_DLY` writer - KERDCG_DLY"]
pub type KERDCG_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DDRCAPBRST` reader - DDRCAPBRST"]
pub type DDRCAPBRST_R = crate::BitReader;
#[doc = "Field `DDRCAPBRST` writer - DDRCAPBRST"]
pub type DDRCAPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCAXIRST` reader - DDRCAXIRST"]
pub type DDRCAXIRST_R = crate::BitReader;
#[doc = "Field `DDRCAXIRST` writer - DDRCAXIRST"]
pub type DDRCAXIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCORERST` reader - DDRCORERST"]
pub type DDRCORERST_R = crate::BitReader;
#[doc = "Field `DDRCORERST` writer - DDRCORERST"]
pub type DDRCORERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHYAPBRST` reader - DPHYAPBRST"]
pub type DPHYAPBRST_R = crate::BitReader;
#[doc = "Field `DPHYAPBRST` writer - DPHYAPBRST"]
pub type DPHYAPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHYRST` reader - DPHYRST"]
pub type DPHYRST_R = crate::BitReader;
#[doc = "Field `DPHYRST` writer - DPHYRST"]
pub type DPHYRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHYCTLRST` reader - DPHYCTLRST"]
pub type DPHYCTLRST_R = crate::BitReader;
#[doc = "Field `DPHYCTLRST` writer - DPHYCTLRST"]
pub type DPHYCTLRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCKMOD` reader - DDRCKMOD"]
pub type DDRCKMOD_R = crate::FieldReader;
#[doc = "Field `DDRCKMOD` writer - DDRCKMOD"]
pub type DDRCKMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GSKPMOD` reader - GSKPMOD"]
pub type GSKPMOD_R = crate::BitReader;
#[doc = "Field `GSKPMOD` writer - GSKPMOD"]
pub type GSKPMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSKPCTRL` reader - GSKPCTRL"]
pub type GSKPCTRL_R = crate::BitReader;
#[doc = "Field `GSKPCTRL` writer - GSKPCTRL"]
pub type GSKPCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFILP_WIDTH` reader - DFILP_WIDTH"]
pub type DFILP_WIDTH_R = crate::FieldReader;
#[doc = "Field `DFILP_WIDTH` writer - DFILP_WIDTH"]
pub type DFILP_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GSKP_DUR` reader - GSKP_DUR"]
pub type GSKP_DUR_R = crate::FieldReader;
#[doc = "Field `GSKP_DUR` writer - GSKP_DUR"]
pub type GSKP_DUR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W<RCC_DDRITFCRrs> {
        DDRC1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W<RCC_DDRITFCRrs> {
        DDRC1LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W<RCC_DDRITFCRrs> {
        DDRC2EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W<RCC_DDRITFCRrs> {
        DDRC2LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W<RCC_DDRITFCRrs> {
        DDRPHYCEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W<RCC_DDRITFCRrs> {
        DDRPHYCLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W<RCC_DDRITFCRrs> {
        DDRCAPBEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W<RCC_DDRITFCRrs> {
        DDRCAPBLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    #[must_use]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W<RCC_DDRITFCRrs> {
        AXIDCGEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W<RCC_DDRITFCRrs> {
        DDRPHYCAPBEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W<RCC_DDRITFCRrs> {
        DDRPHYCAPBLPEN_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    #[must_use]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W<RCC_DDRITFCRrs> {
        KERDCG_DLY_W::new(self, 11)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W<RCC_DDRITFCRrs> {
        DDRCAPBRST_W::new(self, 14)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W<RCC_DDRITFCRrs> {
        DDRCAXIRST_W::new(self, 15)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W<RCC_DDRITFCRrs> {
        DDRCORERST_W::new(self, 16)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W<RCC_DDRITFCRrs> {
        DPHYAPBRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyrst(&mut self) -> DPHYRST_W<RCC_DDRITFCRrs> {
        DPHYRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W<RCC_DDRITFCRrs> {
        DPHYCTLRST_W::new(self, 19)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    #[must_use]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W<RCC_DDRITFCRrs> {
        DDRCKMOD_W::new(self, 20)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn gskpmod(&mut self) -> GSKPMOD_W<RCC_DDRITFCRrs> {
        GSKPMOD_W::new(self, 23)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W<RCC_DDRITFCRrs> {
        GSKPCTRL_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W<RCC_DDRITFCRrs> {
        DFILP_WIDTH_W::new(self, 25)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    #[must_use]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W<RCC_DDRITFCRrs> {
        GSKP_DUR_W::new(self, 28)
    }
}
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ddritfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ddritfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_DDRITFCRrs;
impl crate::RegisterSpec for RCC_DDRITFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ddritfcr::R`](R) reader structure"]
impl crate::Readable for RCC_DDRITFCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ddritfcr::W`](W) writer structure"]
impl crate::Writable for RCC_DDRITFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_DDRITFCR to value 0x000f_d02a"]
impl crate::Resettable for RCC_DDRITFCRrs {
    const RESET_VALUE: u32 = 0x000f_d02a;
}
