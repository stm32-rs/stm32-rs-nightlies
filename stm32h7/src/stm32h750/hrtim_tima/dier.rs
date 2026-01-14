///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///CMP%sDE
pub use crate::stm32h750::hrtim_master::dier::CMP1DE;
///CMP%sIE
pub use crate::stm32h750::hrtim_master::dier::CMP1IE;
///Field `CMPDE(1-4)` reader - CMP%sDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R;
///Field `REPDE` reader - REPDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as REPDE_R;
///Field `UPDDE` reader - UPDDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as UPDDE_R;
///Field `CPTDE(1-2)` reader - CPT%sDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as CPTDE_R;
///Field `SETDE(1-2)` reader - Output %s set DMA request enable
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as SETDE_R;
///Field `RST1DE` reader - RSTx1DE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as RST1DE_R;
///Field `RST2DE` reader - RSTx2DE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as RST2DE_R;
///Field `RSTDE` reader - RSTDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as RSTDE_R;
///Field `DLYPRTDE` reader - DLYPRTDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_R as DLYPRTDE_R;
///Field `CMPDE(1-4)` writer - CMP%sDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W;
///Field `REPDE` writer - REPDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as REPDE_W;
///Field `UPDDE` writer - UPDDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as UPDDE_W;
///Field `CPTDE(1-2)` writer - CPT%sDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as CPTDE_W;
///Field `SETDE(1-2)` writer - Output %s set DMA request enable
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as SETDE_W;
///Field `RST1DE` writer - RSTx1DE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as RST1DE_W;
///Field `RST2DE` writer - RSTx2DE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as RST2DE_W;
///Field `RSTDE` writer - RSTDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as RSTDE_W;
///Field `DLYPRTDE` writer - DLYPRTDE
pub use crate::stm32h750::hrtim_master::dier::CMPDE_W as DLYPRTDE_W;
///Field `CMPIE(1-4)` reader - CMP%sIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R;
///Field `REPIE` reader - REPIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as REPIE_R;
///Field `UPDIE` reader - UPDIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as UPDIE_R;
///Field `CPTIE(1-2)` reader - CPT%sIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as CPTIE_R;
///Field `SETIE(1-2)` reader - Output %s set interrupt enable
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as SETIE_R;
///Field `RST1IE` reader - RSTx1IE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as RST1IE_R;
///Field `RST2IE` reader - RSTx2IE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as RST2IE_R;
///Field `RSTIE` reader - RSTIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as RSTIE_R;
///Field `DLYPRTIE` reader - DLYPRTIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_R as DLYPRTIE_R;
///Field `CMPIE(1-4)` writer - CMP%sIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W;
///Field `REPIE` writer - REPIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as REPIE_W;
///Field `UPDIE` writer - UPDIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as UPDIE_W;
///Field `CPTIE(1-2)` writer - CPT%sIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as CPTIE_W;
///Field `SETIE(1-2)` writer - Output %s set interrupt enable
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as SETIE_W;
///Field `RST1IE` writer - RSTx1IE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as RST1IE_W;
///Field `RST2IE` writer - RSTx2IE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as RST2IE_W;
///Field `RSTIE` writer - RSTIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as RSTIE_W;
///Field `DLYPRTIE` writer - DLYPRTIE
pub use crate::stm32h750::hrtim_master::dier::CMPIE_W as DLYPRTIE_W;
impl R {
    ///CMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&self, n: u8) -> CMPIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///CMP(1-4)IE
    #[inline(always)]
    pub fn cmpie_iter(&self) -> impl Iterator<Item = CMPIE_R> + '_ {
        (0..4).map(move |n| CMPIE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///CPT(1-2)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1IE` field.</div>
    #[inline(always)]
    pub fn cptie(&self, n: u8) -> CPTIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTIE_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CPT(1-2)IE
    #[inline(always)]
    pub fn cptie_iter(&self) -> impl Iterator<Item = CPTIE_R> + '_ {
        (0..2).map(move |n| CPTIE_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPTIE_R {
        CPTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPTIE_R {
        CPTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Output (1-2) set interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1IE` field.</div>
    #[inline(always)]
    pub fn setie(&self, n: u8) -> SETIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETIE_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output (1-2) set interrupt enable
    #[inline(always)]
    pub fn setie_iter(&self) -> impl Iterator<Item = SETIE_R> + '_ {
        (0..2).map(move |n| SETIE_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0))
    }
    ///Bit 9 - Output 1 set interrupt enable
    #[inline(always)]
    pub fn set1ie(&self) -> SETIE_R {
        SETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Output 2 set interrupt enable
    #[inline(always)]
    pub fn set2ie(&self) -> SETIE_R {
        SETIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rst1ie(&self) -> RST1IE_R {
        RST1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rst2ie(&self) -> RST2IE_R {
        RST2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///CMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&self, n: u8) -> CMPDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CMP(1-4)DE
    #[inline(always)]
    pub fn cmpde_iter(&self) -> impl Iterator<Item = CMPDE_R> + '_ {
        (0..4).map(move |n| CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///CPT(1-2)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1DE` field.</div>
    #[inline(always)]
    pub fn cptde(&self, n: u8) -> CPTDE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTDE_R::new(((self.bits >> (n + 23)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CPT(1-2)DE
    #[inline(always)]
    pub fn cptde_iter(&self) -> impl Iterator<Item = CPTDE_R> + '_ {
        (0..2).map(move |n| CPTDE_R::new(((self.bits >> (n + 23)) & 1) != 0))
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&self) -> CPTDE_R {
        CPTDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&self) -> CPTDE_R {
        CPTDE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Output (1-2) set DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1DE` field.</div>
    #[inline(always)]
    pub fn setde(&self, n: u8) -> SETDE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETDE_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output (1-2) set DMA request enable
    #[inline(always)]
    pub fn setde_iter(&self) -> impl Iterator<Item = SETDE_R> + '_ {
        (0..2).map(move |n| SETDE_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0))
    }
    ///Bit 25 - Output 1 set DMA request enable
    #[inline(always)]
    pub fn set1de(&self) -> SETDE_R {
        SETDE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Output 2 set DMA request enable
    #[inline(always)]
    pub fn set2de(&self) -> SETDE_R {
        SETDE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rst1de(&self) -> RST1DE_R {
        RST1DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rst2de(&self) -> RST2DE_R {
        RST2DE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("cmp1de", &self.cmp1de())
            .field("cmp2de", &self.cmp2de())
            .field("cmp3de", &self.cmp3de())
            .field("cmp4de", &self.cmp4de())
            .field("dlyprtde", &self.dlyprtde())
            .field("rstde", &self.rstde())
            .field("rst2de", &self.rst2de())
            .field("set1de", &self.set1de())
            .field("set2de", &self.set2de())
            .field("rst1de", &self.rst1de())
            .field("cpt1de", &self.cpt1de())
            .field("cpt2de", &self.cpt2de())
            .field("updde", &self.updde())
            .field("repde", &self.repde())
            .field("cmp1ie", &self.cmp1ie())
            .field("cmp2ie", &self.cmp2ie())
            .field("cmp3ie", &self.cmp3ie())
            .field("cmp4ie", &self.cmp4ie())
            .field("dlyprtie", &self.dlyprtie())
            .field("rstie", &self.rstie())
            .field("rst2ie", &self.rst2ie())
            .field("set1ie", &self.set1ie())
            .field("set2ie", &self.set2ie())
            .field("rst1ie", &self.rst1ie())
            .field("cpt1ie", &self.cpt1ie())
            .field("cpt2ie", &self.cpt2ie())
            .field("updie", &self.updie())
            .field("repie", &self.repie())
            .finish()
    }
}
impl W {
    ///CMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&mut self, n: u8) -> CMPIE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_W::new(self, n)
    }
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 1)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 2)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&mut self) -> CMPIE_W<'_, DIERrs> {
        CMPIE_W::new(self, 3)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&mut self) -> REPIE_W<'_, DIERrs> {
        REPIE_W::new(self, 4)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W<'_, DIERrs> {
        UPDIE_W::new(self, 6)
    }
    ///CPT(1-2)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1IE` field.</div>
    #[inline(always)]
    pub fn cptie(&mut self, n: u8) -> CPTIE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTIE_W::new(self, n + 7)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&mut self) -> CPTIE_W<'_, DIERrs> {
        CPTIE_W::new(self, 7)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&mut self) -> CPTIE_W<'_, DIERrs> {
        CPTIE_W::new(self, 8)
    }
    ///Output (1-2) set interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1IE` field.</div>
    #[inline(always)]
    pub fn setie(&mut self, n: u8) -> SETIE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETIE_W::new(self, n * 2 + 9)
    }
    ///Bit 9 - Output 1 set interrupt enable
    #[inline(always)]
    pub fn set1ie(&mut self) -> SETIE_W<'_, DIERrs> {
        SETIE_W::new(self, 9)
    }
    ///Bit 11 - Output 2 set interrupt enable
    #[inline(always)]
    pub fn set2ie(&mut self) -> SETIE_W<'_, DIERrs> {
        SETIE_W::new(self, 11)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rst1ie(&mut self) -> RST1IE_W<'_, DIERrs> {
        RST1IE_W::new(self, 10)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rst2ie(&mut self) -> RST2IE_W<'_, DIERrs> {
        RST2IE_W::new(self, 12)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W<'_, DIERrs> {
        RSTIE_W::new(self, 13)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W<'_, DIERrs> {
        DLYPRTIE_W::new(self, 14)
    }
    ///CMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&mut self, n: u8) -> CMPDE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_W::new(self, n + 16)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 16)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 17)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 18)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&mut self) -> CMPDE_W<'_, DIERrs> {
        CMPDE_W::new(self, 19)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&mut self) -> REPDE_W<'_, DIERrs> {
        REPDE_W::new(self, 20)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&mut self) -> UPDDE_W<'_, DIERrs> {
        UPDDE_W::new(self, 22)
    }
    ///CPT(1-2)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1DE` field.</div>
    #[inline(always)]
    pub fn cptde(&mut self, n: u8) -> CPTDE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTDE_W::new(self, n + 23)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&mut self) -> CPTDE_W<'_, DIERrs> {
        CPTDE_W::new(self, 23)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&mut self) -> CPTDE_W<'_, DIERrs> {
        CPTDE_W::new(self, 24)
    }
    ///Output (1-2) set DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1DE` field.</div>
    #[inline(always)]
    pub fn setde(&mut self, n: u8) -> SETDE_W<'_, DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETDE_W::new(self, n * 2 + 25)
    }
    ///Bit 25 - Output 1 set DMA request enable
    #[inline(always)]
    pub fn set1de(&mut self) -> SETDE_W<'_, DIERrs> {
        SETDE_W::new(self, 25)
    }
    ///Bit 27 - Output 2 set DMA request enable
    #[inline(always)]
    pub fn set2de(&mut self) -> SETDE_W<'_, DIERrs> {
        SETDE_W::new(self, 27)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rst1de(&mut self) -> RST1DE_W<'_, DIERrs> {
        RST1DE_W::new(self, 26)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rst2de(&mut self) -> RST2DE_W<'_, DIERrs> {
        RST2DE_W::new(self, 28)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&mut self) -> RSTDE_W<'_, DIERrs> {
        RSTDE_W::new(self, 29)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W<'_, DIERrs> {
        DLYPRTDE_W::new(self, 30)
    }
}
/**TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#HRTIM_TIMA:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
