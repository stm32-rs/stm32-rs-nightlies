#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCRrs>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCRrs>;
#[doc = "Field `SYNCIN` reader - Synchronization inputs"]
pub type SYNCIN_R = crate::FieldReader;
#[doc = "Field `SYNCIN` writer - Synchronization inputs"]
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Synchronization outputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT {
    #[doc = "0: No synchronization output signals. SYNCOUT\\[1:0\\]
should be configured as “No synchronization output signals” when audio block is configured as SPDIF"]
    Disabled = 0,
    #[doc = "1: Block A used for further synchronization for others SAI"]
    BlockA = 1,
    #[doc = "2: Block B used for further synchronization for others SAI"]
    BlockB = 2,
}
impl From<SYNCOUT> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCOUT {
    type Ux = u8;
}
#[doc = "Field `SYNCOUT` reader - Synchronization outputs"]
pub type SYNCOUT_R = crate::FieldReader<SYNCOUT>;
impl SYNCOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCOUT> {
        match self.bits {
            0 => Some(SYNCOUT::Disabled),
            1 => Some(SYNCOUT::BlockA),
            2 => Some(SYNCOUT::BlockB),
            _ => None,
        }
    }
    #[doc = "No synchronization output signals. SYNCOUT\\[1:0\\]
should be configured as “No synchronization output signals” when audio block is configured as SPDIF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT::Disabled
    }
    #[doc = "Block A used for further synchronization for others SAI"]
    #[inline(always)]
    pub fn is_block_a(&self) -> bool {
        *self == SYNCOUT::BlockA
    }
    #[doc = "Block B used for further synchronization for others SAI"]
    #[inline(always)]
    pub fn is_block_b(&self) -> bool {
        *self == SYNCOUT::BlockB
    }
}
#[doc = "Field `SYNCOUT` writer - Synchronization outputs"]
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCOUT>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No synchronization output signals. SYNCOUT\\[1:0\\]
should be configured as “No synchronization output signals” when audio block is configured as SPDIF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::Disabled)
    }
    #[doc = "Block A used for further synchronization for others SAI"]
    #[inline(always)]
    pub fn block_a(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::BlockA)
    }
    #[doc = "Block B used for further synchronization for others SAI"]
    #[inline(always)]
    pub fn block_b(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::BlockB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization outputs"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<GCRrs> {
        SYNCIN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Synchronization outputs"]
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<GCRrs> {
        SYNCOUT_W::new(self, 4)
    }
}
#[doc = "Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCRrs {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0;
}
