///Register `CCMR1_Output` reader
pub type R = crate::R<CCMR1_OUTPUTrs>;
///Register `CCMR1_Output` writer
pub type W = crate::W<CCMR1_OUTPUTrs>;
///Field `CCS(1-2)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader;
///Field `CCS(1-2)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OCFE(1-2)` reader - Output compare %s fast enable
pub type OCFE_R = crate::BitReader;
///Field `OCFE(1-2)` writer - Output compare %s fast enable
pub type OCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCPE(1-2)` reader - Output compare %s preload enable
pub type OCPE_R = crate::BitReader;
///Field `OCPE(1-2)` writer - Output compare %s preload enable
pub type OCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCM(1-2)` reader - Output compare %s mode
pub type OCM_R = crate::FieldReader;
///Field `OCM(1-2)` writer - Output compare %s mode
pub type OCM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OCCE(1-2)` reader - Output compare %s clear enable
pub type OCCE_R = crate::BitReader;
///Field `OCCE(1-2)` writer - Output compare %s clear enable
pub type OCCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCM_3(1-2)` reader - Output compare %s mode, bit 3
pub type OCM_3_R = crate::BitReader;
///Field `OCM_3(1-2)` writer - Output compare %s mode, bit 3
pub type OCM_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Capture/Compare (1-2) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-2) selection
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Output compare (1-2) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-2) fast enable
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Output compare (1-2) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-2) preload enable
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Output compare (1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Output compare (1-2) mode
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..2).map(move |n| OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Output compare (1-2) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1CE` field.</div>
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-2) clear enable
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..2).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Output compare (1-2) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (1-2) mode, bit 3
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..2).map(move |n| OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0))
    }
    ///Bit 16 - Output compare 1 mode, bit 3
    #[inline(always)]
    pub fn oc1m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 2 mode, bit 3
    #[inline(always)]
    pub fn oc2m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Output")
            .field("cc1s", &self.cc1s())
            .field("cc2s", &self.cc2s())
            .field("oc1fe", &self.oc1fe())
            .field("oc2fe", &self.oc2fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc2pe", &self.oc2pe())
            .field("oc1m", &self.oc1m())
            .field("oc2m", &self.oc2m())
            .field("oc1ce", &self.oc1ce())
            .field("oc2ce", &self.oc2ce())
            .field("oc1m_3", &self.oc1m_3())
            .field("oc2m_3", &self.oc2m_3())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-2) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&mut self) -> CCS_W<CCMR1_OUTPUTrs> {
        CCS_W::new(self, 8)
    }
    ///Output compare (1-2) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    ///Bit 2 - Output compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    ///Bit 10 - Output compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OCFE_W<CCMR1_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    ///Output compare (1-2) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    ///Bit 3 - Output compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    ///Bit 11 - Output compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OCPE_W<CCMR1_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    ///Output compare (1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M` field.</div>
    #[inline(always)]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_W::new(self, n * 8 + 4)
    }
    ///Bits 4:6 - Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    ///Bits 12:14 - Output compare 2 mode
    #[inline(always)]
    pub fn oc2m(&mut self) -> OCM_W<CCMR1_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    ///Output compare (1-2) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1CE` field.</div>
    #[inline(always)]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_W::new(self, n * 8 + 7)
    }
    ///Bit 7 - Output compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OCCE_W<CCMR1_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    ///Bit 15 - Output compare 2 clear enable
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OCCE_W<CCMR1_OUTPUTrs> {
        OCCE_W::new(self, 15)
    }
    ///Output compare (1-2) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC1M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR1_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_W::new(self, n * 8 + 16)
    }
    ///Bit 16 - Output compare 1 mode, bit 3
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> OCM_3_W<CCMR1_OUTPUTrs> {
        OCM_3_W::new(self, 16)
    }
    ///Bit 24 - Output compare 2 mode, bit 3
    #[inline(always)]
    pub fn oc2m_3(&mut self) -> OCM_3_W<CCMR1_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
/**TIM3 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#TIM3:CCMR1_Output)*/
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_output::R`](R) reader structure
impl crate::Readable for CCMR1_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
