#[doc = "Register `CCCSR` reader"]
pub type R = crate::R<CCCSRrs>;
#[doc = "Register `CCCSR` writer"]
pub type W = crate::W<CCCSRrs>;
#[doc = "enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    #[doc = "0: I/O compensation cell disabled"]
    Disabled = 0,
    #[doc = "1: I/O compensation cell enabled"]
    Enabled = 1,
}
impl From<EN1> for bool {
    #[inline(always)]
    fn from(variant: EN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_R = crate::BitReader<EN1>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN1 {
        match self.bits {
            false => EN1::Disabled,
            true => EN1::Enabled,
        }
    }
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1::Enabled
    }
}
#[doc = "Field `EN1` writer - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
#[doc = "code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CS1 {
    #[doc = "0: Code from cell selected"]
    Cell = 0,
    #[doc = "1: Code from CCSWCR selected"]
    Ccswcr = 1,
}
impl From<CS1> for bool {
    #[inline(always)]
    fn from(variant: CS1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS1` reader - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_R = crate::BitReader<CS1>;
impl CS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS1 {
        match self.bits {
            false => CS1::Cell,
            true => CS1::Ccswcr,
        }
    }
    #[doc = "Code from cell selected"]
    #[inline(always)]
    pub fn is_cell(&self) -> bool {
        *self == CS1::Cell
    }
    #[doc = "Code from CCSWCR selected"]
    #[inline(always)]
    pub fn is_ccswcr(&self) -> bool {
        *self == CS1::Ccswcr
    }
}
#[doc = "Field `CS1` writer - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_W<'a, REG> = crate::BitWriter<'a, REG, CS1>;
impl<'a, REG> CS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code from cell selected"]
    #[inline(always)]
    pub fn cell(self) -> &'a mut crate::W<REG> {
        self.variant(CS1::Cell)
    }
    #[doc = "Code from CCSWCR selected"]
    #[inline(always)]
    pub fn ccswcr(self) -> &'a mut crate::W<REG> {
        self.variant(CS1::Ccswcr)
    }
}
#[doc = "Field `CS2` reader - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub use CS1_R as CS2_R;
#[doc = "Field `CS2` writer - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub use CS1_W as CS2_W;
#[doc = "Field `EN2` reader - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub use EN1_R as EN2_R;
#[doc = "Field `EN2` writer - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub use EN1_W as EN2_W;
#[doc = "VDDIO compensation cell ready flag This bit provides the status of the compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY1R {
    #[doc = "0: VDDIO compensation cell not ready"]
    NotReady = 0,
    #[doc = "1: VDDIO compensation cell ready"]
    Ready = 1,
}
impl From<RDY1R> for bool {
    #[inline(always)]
    fn from(variant: RDY1R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY1` reader - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
pub type RDY1_R = crate::BitReader<RDY1R>;
impl RDY1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY1R {
        match self.bits {
            false => RDY1R::NotReady,
            true => RDY1R::Ready,
        }
    }
    #[doc = "VDDIO compensation cell not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RDY1R::NotReady
    }
    #[doc = "VDDIO compensation cell ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY1R::Ready
    }
}
#[doc = "Field `RDY2` reader - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
pub use RDY1_R as RDY2_R;
impl R {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CCCSRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<CCCSRrs> {
        CS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CCCSRrs> {
        EN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<CCCSRrs> {
        CS2_W::new(self, 3)
    }
}
#[doc = "SBS compensation cell for I/Os control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccsr::R`](R) reader structure"]
impl crate::Readable for CCCSRrs {}
#[doc = "`write(|w| ..)` method takes [`cccsr::W`](W) writer structure"]
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CCCSRrs {
    const RESET_VALUE: u32 = 0;
}
