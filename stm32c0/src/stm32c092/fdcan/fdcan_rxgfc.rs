///Register `FDCAN_RXGFC` reader
pub type R = crate::R<FDCAN_RXGFCrs>;
///Register `FDCAN_RXGFC` writer
pub type W = crate::W<FDCAN_RXGFCrs>;
/**Reject remote frames extended

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFE {
    ///0: Filter remote frames with 29-bit standard IDs
    B0x0 = 0,
    ///1: Reject all remote frames with 29-bit standard IDs
    B0x1 = 1,
}
impl From<RRFE> for bool {
    #[inline(always)]
    fn from(variant: RRFE) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFE` reader - Reject remote frames extended
pub type RRFE_R = crate::BitReader<RRFE>;
impl RRFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRFE {
        match self.bits {
            false => RRFE::B0x0,
            true => RRFE::B0x1,
        }
    }
    ///Filter remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFE::B0x0
    }
    ///Reject all remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFE::B0x1
    }
}
///Field `RRFE` writer - Reject remote frames extended
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG, RRFE>;
impl<'a, REG> RRFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filter remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFE::B0x0)
    }
    ///Reject all remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFE::B0x1)
    }
}
/**Reject remote frames standard

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFS {
    ///0: Filter remote frames with 11-bit standard IDs
    B0x0 = 0,
    ///1: Reject all remote frames with 11-bit standard IDs
    B0x1 = 1,
}
impl From<RRFS> for bool {
    #[inline(always)]
    fn from(variant: RRFS) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFS` reader - Reject remote frames standard
pub type RRFS_R = crate::BitReader<RRFS>;
impl RRFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRFS {
        match self.bits {
            false => RRFS::B0x0,
            true => RRFS::B0x1,
        }
    }
    ///Filter remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFS::B0x0
    }
    ///Reject all remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFS::B0x1
    }
}
///Field `RRFS` writer - Reject remote frames standard
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG, RRFS>;
impl<'a, REG> RRFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filter remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFS::B0x0)
    }
    ///Reject all remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFS::B0x1)
    }
}
/**Accept non-matching frames extended

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFE {
    ///0: Accept in Rx FIFO 0
    B0x0 = 0,
    ///1: Accept in Rx FIFO 1
    B0x1 = 1,
    ///2: Reject
    B0x2 = 2,
    ///3: Reject
    B0x3 = 3,
}
impl From<ANFE> for u8 {
    #[inline(always)]
    fn from(variant: ANFE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFE {
    type Ux = u8;
}
impl crate::IsEnum for ANFE {}
///Field `ANFE` reader - Accept non-matching frames extended
pub type ANFE_R = crate::FieldReader<ANFE>;
impl ANFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANFE {
        match self.bits {
            0 => ANFE::B0x0,
            1 => ANFE::B0x1,
            2 => ANFE::B0x2,
            3 => ANFE::B0x3,
            _ => unreachable!(),
        }
    }
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ANFE::B0x0
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ANFE::B0x1
    }
    ///Reject
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ANFE::B0x2
    }
    ///Reject
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ANFE::B0x3
    }
}
///Field `ANFE` writer - Accept non-matching frames extended
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANFE, crate::Safe>;
impl<'a, REG> ANFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE::B0x0)
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE::B0x1)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE::B0x2)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE::B0x3)
    }
}
/**Accept Non-matching frames standard

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFS {
    ///0: Accept in Rx FIFO 0
    B0x0 = 0,
    ///1: Accept in Rx FIFO 1
    B0x1 = 1,
    ///2: Reject
    B0x2 = 2,
    ///3: Reject
    B0x3 = 3,
}
impl From<ANFS> for u8 {
    #[inline(always)]
    fn from(variant: ANFS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFS {
    type Ux = u8;
}
impl crate::IsEnum for ANFS {}
///Field `ANFS` reader - Accept Non-matching frames standard
pub type ANFS_R = crate::FieldReader<ANFS>;
impl ANFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANFS {
        match self.bits {
            0 => ANFS::B0x0,
            1 => ANFS::B0x1,
            2 => ANFS::B0x2,
            3 => ANFS::B0x3,
            _ => unreachable!(),
        }
    }
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ANFS::B0x0
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ANFS::B0x1
    }
    ///Reject
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ANFS::B0x2
    }
    ///Reject
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ANFS::B0x3
    }
}
///Field `ANFS` writer - Accept Non-matching frames standard
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANFS, crate::Safe>;
impl<'a, REG> ANFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS::B0x0)
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS::B0x1)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS::B0x2)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS::B0x3)
    }
}
///Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking)
pub type F1OM_R = crate::BitReader;
///Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking)
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking)
pub type F0OM_R = crate::BitReader;
///Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking)
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Number of standard filter elements in the list

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSS {
    ///0: No standard message ID filter
    B0x0 = 0,
}
impl From<LSS> for u8 {
    #[inline(always)]
    fn from(variant: LSS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSS {
    type Ux = u8;
}
impl crate::IsEnum for LSS {}
///Field `LSS` reader - Number of standard filter elements in the list
pub type LSS_R = crate::FieldReader<LSS>;
impl LSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSS> {
        match self.bits {
            0 => Some(LSS::B0x0),
            _ => None,
        }
    }
    ///No standard message ID filter
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSS::B0x0
    }
}
///Field `LSS` writer - Number of standard filter elements in the list
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5, LSS>;
impl<'a, REG> LSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No standard message ID filter
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSS::B0x0)
    }
}
/**Number of extended filter elements in the list

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSE {
    ///0: No extended message ID filter
    B0x0 = 0,
}
impl From<LSE> for u8 {
    #[inline(always)]
    fn from(variant: LSE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSE {
    type Ux = u8;
}
impl crate::IsEnum for LSE {}
///Field `LSE` reader - Number of extended filter elements in the list
pub type LSE_R = crate::FieldReader<LSE>;
impl LSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSE> {
        match self.bits {
            0 => Some(LSE::B0x0),
            _ => None,
        }
    }
    ///No extended message ID filter
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSE::B0x0
    }
}
///Field `LSE` writer - Number of extended filter elements in the list
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LSE>;
impl<'a, REG> LSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No extended message ID filter
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSE::B0x0)
    }
}
impl R {
    ///Bit 0 - Reject remote frames extended
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reject remote frames standard
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Accept non-matching frames extended
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Accept Non-matching frames standard
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking)
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking)
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:20 - Number of standard filter elements in the list
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - Number of extended filter elements in the list
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXGFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .field("f1om", &self.f1om())
            .field("f0om", &self.f0om())
            .field("lss", &self.lss())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reject remote frames extended
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W<'_, FDCAN_RXGFCrs> {
        RRFE_W::new(self, 0)
    }
    ///Bit 1 - Reject remote frames standard
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W<'_, FDCAN_RXGFCrs> {
        RRFS_W::new(self, 1)
    }
    ///Bits 2:3 - Accept non-matching frames extended
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W<'_, FDCAN_RXGFCrs> {
        ANFE_W::new(self, 2)
    }
    ///Bits 4:5 - Accept Non-matching frames standard
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W<'_, FDCAN_RXGFCrs> {
        ANFS_W::new(self, 4)
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking)
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W<'_, FDCAN_RXGFCrs> {
        F1OM_W::new(self, 8)
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking)
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W<'_, FDCAN_RXGFCrs> {
        F0OM_W::new(self, 9)
    }
    ///Bits 16:20 - Number of standard filter elements in the list
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W<'_, FDCAN_RXGFCrs> {
        LSS_W::new(self, 16)
    }
    ///Bits 24:27 - Number of extended filter elements in the list
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W<'_, FDCAN_RXGFCrs> {
        LSE_W::new(self, 24)
    }
}
/**FDCAN global filter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxgfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_RXGFC)*/
pub struct FDCAN_RXGFCrs;
impl crate::RegisterSpec for FDCAN_RXGFCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxgfc::R`](R) reader structure
impl crate::Readable for FDCAN_RXGFCrs {}
///`write(|w| ..)` method takes [`fdcan_rxgfc::W`](W) writer structure
impl crate::Writable for FDCAN_RXGFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_RXGFC to value 0
impl crate::Resettable for FDCAN_RXGFCrs {}
