///Register `DDRITFCR` reader
pub type R = crate::R<DDRITFCRrs>;
///Register `DDRITFCR` writer
pub type W = crate::W<DDRITFCRrs>;
///Field `DDRC1EN` reader - DDRC1EN
pub type DDRC1EN_R = crate::BitReader;
///Field `DDRC1EN` writer - DDRC1EN
pub type DDRC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRC1LPEN` reader - DDRC1LPEN
pub type DDRC1LPEN_R = crate::BitReader;
///Field `DDRC1LPEN` writer - DDRC1LPEN
pub type DDRC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRC2EN` reader - DDRC2EN
pub type DDRC2EN_R = crate::BitReader;
///Field `DDRC2EN` writer - DDRC2EN
pub type DDRC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRC2LPEN` reader - DDRC2LPEN
pub type DDRC2LPEN_R = crate::BitReader;
///Field `DDRC2LPEN` writer - DDRC2LPEN
pub type DDRC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPHYCEN` reader - DDRPHYCEN
pub type DDRPHYCEN_R = crate::BitReader;
///Field `DDRPHYCEN` writer - DDRPHYCEN
pub type DDRPHYCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPHYCLPEN` reader - DDRPHYCLPEN
pub type DDRPHYCLPEN_R = crate::BitReader;
///Field `DDRPHYCLPEN` writer - DDRPHYCLPEN
pub type DDRPHYCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRCAPBEN` reader - DDRCAPBEN
pub type DDRCAPBEN_R = crate::BitReader;
///Field `DDRCAPBEN` writer - DDRCAPBEN
pub type DDRCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRCAPBLPEN` reader - DDRCAPBLPEN
pub type DDRCAPBLPEN_R = crate::BitReader;
///Field `DDRCAPBLPEN` writer - DDRCAPBLPEN
pub type DDRCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIDCGEN` reader - AXIDCGEN
pub type AXIDCGEN_R = crate::BitReader;
///Field `AXIDCGEN` writer - AXIDCGEN
pub type AXIDCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPHYCAPBEN` reader - DDRPHYCAPBEN
pub type DDRPHYCAPBEN_R = crate::BitReader;
///Field `DDRPHYCAPBEN` writer - DDRPHYCAPBEN
pub type DDRPHYCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPHYCAPBLPEN` reader - DDRPHYCAPBLPEN
pub type DDRPHYCAPBLPEN_R = crate::BitReader;
///Field `DDRPHYCAPBLPEN` writer - DDRPHYCAPBLPEN
pub type DDRPHYCAPBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KERDCG_DLY` reader - KERDCG_DLY
pub type KERDCG_DLY_R = crate::FieldReader;
///Field `KERDCG_DLY` writer - KERDCG_DLY
pub type KERDCG_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DDRCAPBRST` reader - DDRCAPBRST
pub type DDRCAPBRST_R = crate::BitReader;
///Field `DDRCAPBRST` writer - DDRCAPBRST
pub type DDRCAPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRCAXIRST` reader - DDRCAXIRST
pub type DDRCAXIRST_R = crate::BitReader;
///Field `DDRCAXIRST` writer - DDRCAXIRST
pub type DDRCAXIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRCORERST` reader - DDRCORERST
pub type DDRCORERST_R = crate::BitReader;
///Field `DDRCORERST` writer - DDRCORERST
pub type DDRCORERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPHYAPBRST` reader - DPHYAPBRST
pub type DPHYAPBRST_R = crate::BitReader;
///Field `DPHYAPBRST` writer - DPHYAPBRST
pub type DPHYAPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPHYRST` reader - DPHYRST
pub type DPHYRST_R = crate::BitReader;
///Field `DPHYRST` writer - DPHYRST
pub type DPHYRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPHYCTLRST` reader - DPHYCTLRST
pub type DPHYCTLRST_R = crate::BitReader;
///Field `DPHYCTLRST` writer - DPHYCTLRST
pub type DPHYCTLRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRCKMOD` reader - DDRCKMOD
pub type DDRCKMOD_R = crate::FieldReader;
///Field `DDRCKMOD` writer - DDRCKMOD
pub type DDRCKMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GSKPMOD` reader - GSKPMOD
pub type GSKPMOD_R = crate::BitReader;
///Field `GSKPMOD` writer - GSKPMOD
pub type GSKPMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSKPCTRL` reader - GSKPCTRL
pub type GSKPCTRL_R = crate::BitReader;
///Field `GSKPCTRL` writer - GSKPCTRL
pub type GSKPCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFILP_WIDTH` reader - DFILP_WIDTH
pub type DFILP_WIDTH_R = crate::FieldReader;
///Field `DFILP_WIDTH` writer - DFILP_WIDTH
pub type DFILP_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GSKP_DUR` reader - GSKP_DUR
pub type GSKP_DUR_R = crate::FieldReader;
///Field `GSKP_DUR` writer - GSKP_DUR
pub type GSKP_DUR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - DDRC1EN
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DDRC1LPEN
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DDRC2EN
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DDRC2LPEN
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DDRPHYCEN
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DDRPHYCLPEN
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DDRCAPBEN
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DDRCAPBLPEN
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXIDCGEN
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DDRPHYCAPBEN
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DDRPHYCAPBLPEN
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - KERDCG_DLY
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - DDRCAPBRST
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DDRCAXIRST
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DDRCORERST
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DPHYAPBRST
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DPHYRST
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DPHYCTLRST
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:22 - DDRCKMOD
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - GSKPMOD
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GSKPCTRL
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - DFILP_WIDTH
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:31 - GSKP_DUR
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRITFCR")
            .field("ddrc1en", &self.ddrc1en())
            .field("ddrc1lpen", &self.ddrc1lpen())
            .field("ddrc2en", &self.ddrc2en())
            .field("ddrc2lpen", &self.ddrc2lpen())
            .field("ddrphycen", &self.ddrphycen())
            .field("ddrphyclpen", &self.ddrphyclpen())
            .field("ddrcapben", &self.ddrcapben())
            .field("ddrcapblpen", &self.ddrcapblpen())
            .field("axidcgen", &self.axidcgen())
            .field("ddrphycapben", &self.ddrphycapben())
            .field("ddrphycapblpen", &self.ddrphycapblpen())
            .field("kerdcg_dly", &self.kerdcg_dly())
            .field("ddrcapbrst", &self.ddrcapbrst())
            .field("ddrcaxirst", &self.ddrcaxirst())
            .field("ddrcorerst", &self.ddrcorerst())
            .field("dphyapbrst", &self.dphyapbrst())
            .field("dphyrst", &self.dphyrst())
            .field("dphyctlrst", &self.dphyctlrst())
            .field("ddrckmod", &self.ddrckmod())
            .field("gskpmod", &self.gskpmod())
            .field("gskpctrl", &self.gskpctrl())
            .field("dfilp_width", &self.dfilp_width())
            .field("gskp_dur", &self.gskp_dur())
            .finish()
    }
}
impl W {
    ///Bit 0 - DDRC1EN
    #[inline(always)]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W<'_, DDRITFCRrs> {
        DDRC1EN_W::new(self, 0)
    }
    ///Bit 1 - DDRC1LPEN
    #[inline(always)]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W<'_, DDRITFCRrs> {
        DDRC1LPEN_W::new(self, 1)
    }
    ///Bit 2 - DDRC2EN
    #[inline(always)]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W<'_, DDRITFCRrs> {
        DDRC2EN_W::new(self, 2)
    }
    ///Bit 3 - DDRC2LPEN
    #[inline(always)]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W<'_, DDRITFCRrs> {
        DDRC2LPEN_W::new(self, 3)
    }
    ///Bit 4 - DDRPHYCEN
    #[inline(always)]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W<'_, DDRITFCRrs> {
        DDRPHYCEN_W::new(self, 4)
    }
    ///Bit 5 - DDRPHYCLPEN
    #[inline(always)]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W<'_, DDRITFCRrs> {
        DDRPHYCLPEN_W::new(self, 5)
    }
    ///Bit 6 - DDRCAPBEN
    #[inline(always)]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W<'_, DDRITFCRrs> {
        DDRCAPBEN_W::new(self, 6)
    }
    ///Bit 7 - DDRCAPBLPEN
    #[inline(always)]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W<'_, DDRITFCRrs> {
        DDRCAPBLPEN_W::new(self, 7)
    }
    ///Bit 8 - AXIDCGEN
    #[inline(always)]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W<'_, DDRITFCRrs> {
        AXIDCGEN_W::new(self, 8)
    }
    ///Bit 9 - DDRPHYCAPBEN
    #[inline(always)]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W<'_, DDRITFCRrs> {
        DDRPHYCAPBEN_W::new(self, 9)
    }
    ///Bit 10 - DDRPHYCAPBLPEN
    #[inline(always)]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W<'_, DDRITFCRrs> {
        DDRPHYCAPBLPEN_W::new(self, 10)
    }
    ///Bits 11:13 - KERDCG_DLY
    #[inline(always)]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W<'_, DDRITFCRrs> {
        KERDCG_DLY_W::new(self, 11)
    }
    ///Bit 14 - DDRCAPBRST
    #[inline(always)]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W<'_, DDRITFCRrs> {
        DDRCAPBRST_W::new(self, 14)
    }
    ///Bit 15 - DDRCAXIRST
    #[inline(always)]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W<'_, DDRITFCRrs> {
        DDRCAXIRST_W::new(self, 15)
    }
    ///Bit 16 - DDRCORERST
    #[inline(always)]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W<'_, DDRITFCRrs> {
        DDRCORERST_W::new(self, 16)
    }
    ///Bit 17 - DPHYAPBRST
    #[inline(always)]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W<'_, DDRITFCRrs> {
        DPHYAPBRST_W::new(self, 17)
    }
    ///Bit 18 - DPHYRST
    #[inline(always)]
    pub fn dphyrst(&mut self) -> DPHYRST_W<'_, DDRITFCRrs> {
        DPHYRST_W::new(self, 18)
    }
    ///Bit 19 - DPHYCTLRST
    #[inline(always)]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W<'_, DDRITFCRrs> {
        DPHYCTLRST_W::new(self, 19)
    }
    ///Bits 20:22 - DDRCKMOD
    #[inline(always)]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W<'_, DDRITFCRrs> {
        DDRCKMOD_W::new(self, 20)
    }
    ///Bit 23 - GSKPMOD
    #[inline(always)]
    pub fn gskpmod(&mut self) -> GSKPMOD_W<'_, DDRITFCRrs> {
        GSKPMOD_W::new(self, 23)
    }
    ///Bit 24 - GSKPCTRL
    #[inline(always)]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W<'_, DDRITFCRrs> {
        GSKPCTRL_W::new(self, 24)
    }
    ///Bits 25:27 - DFILP_WIDTH
    #[inline(always)]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W<'_, DDRITFCRrs> {
        DFILP_WIDTH_W::new(self, 25)
    }
    ///Bits 28:31 - GSKP_DUR
    #[inline(always)]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W<'_, DDRITFCRrs> {
        GSKP_DUR_W::new(self, 28)
    }
}
/**This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`ddritfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddritfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:DDRITFCR)*/
pub struct DDRITFCRrs;
impl crate::RegisterSpec for DDRITFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ddritfcr::R`](R) reader structure
impl crate::Readable for DDRITFCRrs {}
///`write(|w| ..)` method takes [`ddritfcr::W`](W) writer structure
impl crate::Writable for DDRITFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DDRITFCR to value 0x000f_d02a
impl crate::Resettable for DDRITFCRrs {
    const RESET_VALUE: u32 = 0x000f_d02a;
}
