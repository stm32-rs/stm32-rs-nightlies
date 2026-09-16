///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
///Field `SYNCIN` reader - Synchronization inputs
pub type SYNCIN_R = crate::FieldReader;
///Field `SYNCIN` writer - Synchronization inputs
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Synchronization outputs

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT {
    ///0: No synchronization output signals. SYNCOUT\[1:0\] should be configured as “No synchronization output signals” when audio block is configured as SPDIF
    Disabled = 0,
    ///1: Block A used for further synchronization for others SAI
    BlockA = 1,
    ///2: Block B used for further synchronization for others SAI
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
impl crate::IsEnum for SYNCOUT {}
///Field `SYNCOUT` reader - Synchronization outputs
pub type SYNCOUT_R = crate::FieldReader<SYNCOUT>;
impl SYNCOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCOUT> {
        match self.bits {
            0 => Some(SYNCOUT::Disabled),
            1 => Some(SYNCOUT::BlockA),
            2 => Some(SYNCOUT::BlockB),
            _ => None,
        }
    }
    ///No synchronization output signals. SYNCOUT\[1:0\] should be configured as “No synchronization output signals” when audio block is configured as SPDIF
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT::Disabled
    }
    ///Block A used for further synchronization for others SAI
    #[inline(always)]
    pub fn is_block_a(&self) -> bool {
        *self == SYNCOUT::BlockA
    }
    ///Block B used for further synchronization for others SAI
    #[inline(always)]
    pub fn is_block_b(&self) -> bool {
        *self == SYNCOUT::BlockB
    }
}
///Field `SYNCOUT` writer - Synchronization outputs
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCOUT>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No synchronization output signals. SYNCOUT\[1:0\] should be configured as “No synchronization output signals” when audio block is configured as SPDIF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::Disabled)
    }
    ///Block A used for further synchronization for others SAI
    #[inline(always)]
    pub fn block_a(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::BlockA)
    }
    ///Block B used for further synchronization for others SAI
    #[inline(always)]
    pub fn block_b(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::BlockB)
    }
}
impl R {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("syncout", &self.syncout())
            .field("syncin", &self.syncin())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Synchronization inputs
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W<'_, GCRrs> {
        SYNCIN_W::new(self, 0)
    }
    ///Bits 4:5 - Synchronization outputs
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W<'_, GCRrs> {
        SYNCOUT_W::new(self, 4)
    }
}
/**Global configuration register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SAI1:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCRrs {}
